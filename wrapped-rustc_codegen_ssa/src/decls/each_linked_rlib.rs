macro_rules! deps {
    () => {
        CrateInfo!();
        LinkRlibError!();
    };
}

macro_rules! each_linked_rlib {
    () => {
        deps!();
        pub fn each_linked_rlib (info : & CrateInfo , crate_type : Option < CrateType > , f : & mut dyn FnMut (CrateNum , & Path) ,) -> Result < () , errors :: LinkRlibError > { let fmts = if let Some (crate_type) = crate_type { let Some (fmts) = info . dependency_formats . get (& crate_type) else { return Err (errors :: LinkRlibError :: MissingFormat) ; } ; fmts } else { let mut dep_formats = info . dependency_formats . iter () ; let (ty1 , list1) = dep_formats . next () . ok_or (errors :: LinkRlibError :: MissingFormat) ? ; if let Some ((ty2 , list2)) = dep_formats . find (| (_ , list2) | list1 != * list2) { return Err (errors :: LinkRlibError :: IncompatibleDependencyFormats { ty1 : format ! ("{ty1:?}") , ty2 : format ! ("{ty2:?}") , list1 : format ! ("{list1:?}") , list2 : format ! ("{list2:?}") , }) ; } list1 } ; let used_dep_crates = info . used_crates . iter () ; for & cnum in used_dep_crates { match fmts . get (cnum) { Some (& Linkage :: NotLinked | & Linkage :: Dynamic | & Linkage :: IncludedFromDylib) => continue , Some (_) => { } None => return Err (errors :: LinkRlibError :: MissingFormat) , } let crate_name = info . crate_name [& cnum] ; let used_crate_source = & info . used_crate_source [& cnum] ; if let Some ((path , _)) = & used_crate_source . rlib { f (cnum , path) ; } else if used_crate_source . rmeta . is_some () { return Err (errors :: LinkRlibError :: OnlyRmetaFound { crate_name }) ; } else { return Err (errors :: LinkRlibError :: NotFound { crate_name }) ; } } Ok (()) }
    };
}

each_linked_rlib!();