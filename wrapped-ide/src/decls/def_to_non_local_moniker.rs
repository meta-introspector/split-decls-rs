macro_rules! deps {
    () => {
        MonikerDescriptor!();
        PackageInformation!();
        MonikerKind!();
        Moniker!();
        MonikerIdentifier!();
        MonikerDescriptorKind!();
    };
}

macro_rules! def_to_non_local_moniker {
    () => {
        deps!();
        fn def_to_non_local_moniker (db : & RootDatabase , definition : Definition , from_crate : Crate ,) -> Option < Moniker > { let module = match definition { Definition :: Module (module) if module . is_crate_root () => module , _ => definition . module (db) ? , } ; let krate = module . krate () ; let edition = krate . edition (db) ; let mut reverse_description = vec ! [] ; let mut def = definition ; loop { match def { Definition :: SelfType (impl_) => { if let Some (trait_ref) = impl_ . trait_ref (db) { reverse_description . push (MonikerDescriptor { name : display (db , module , trait_ref) , desc : MonikerDescriptorKind :: TypeParameter , }) ; } reverse_description . push (MonikerDescriptor { name : display (db , module , impl_ . self_ty (db)) , desc : MonikerDescriptorKind :: TypeParameter , }) ; reverse_description . push (MonikerDescriptor { name : "impl" . to_owned () , desc : MonikerDescriptorKind :: Type , }) ; } _ => { if let Some (name) = def . name (db) { reverse_description . push (MonikerDescriptor { name : name . display (db , edition) . to_string () , desc : def_to_kind (db , def) . into () , }) ; } else { match def { Definition :: Module (module) if module . is_crate_root () => { if reverse_description . is_empty () { reverse_description . push (MonikerDescriptor { name : "crate" . to_owned () , desc : MonikerDescriptorKind :: Namespace , }) ; } } _ => { tracing :: error ! (? def , "Encountered enclosing definition with no name") ; } } } } } let Some (next_def) = def . enclosing_definition (db) else { break ; } ; def = next_def ; } if reverse_description . is_empty () { return None ; } reverse_description . reverse () ; let description = reverse_description ; Some (Moniker { identifier : MonikerIdentifier { crate_name : krate . display_name (db) ? . crate_name () . to_string () , description , } , kind : if krate == from_crate { MonikerKind :: Export } else { MonikerKind :: Import } , package_information : { let (name , repo , version) = match krate . origin (db) { CrateOrigin :: Library { repo , name } => (name , repo , krate . version (db)) , CrateOrigin :: Local { repo , name } => (name . unwrap_or (krate . display_name (db) ? . canonical_name () . to_owned ()) , repo , krate . version (db) ,) , CrateOrigin :: Rustc { name } => (name . clone () , Some ("https://github.com/rust-lang/rust/" . to_owned ()) , Some (format ! ("https://github.com/rust-lang/rust/compiler/{name}" ,)) ,) , CrateOrigin :: Lang (lang) => (krate . display_name (db) ? . canonical_name () . to_owned () , Some ("https://github.com/rust-lang/rust/" . to_owned ()) , Some (match lang { LangCrateOrigin :: Other => { "https://github.com/rust-lang/rust/library/" . into () } lang => format ! ("https://github.com/rust-lang/rust/library/{lang}" ,) , }) ,) , } ; PackageInformation { name : name . as_str () . to_owned () , repo , version } } , }) }
    };
}

def_to_non_local_moniker!();