macro_rules! deps {
    () => {
        ModuleDefId!();
        DefDatabase!();
    };
}

macro_rules! crate_notable_traits {
    () => {
        deps!();
        # [salsa :: tracked (returns (as_deref))] pub (crate) fn crate_notable_traits (db : & dyn DefDatabase , krate : Crate) -> Option < Box < [TraitId] > > { let mut traits = Vec :: new () ; let crate_def_map = crate_def_map (db , krate) ; for (_ , module_data) in crate_def_map . modules () { for def in module_data . scope . declarations () { if let ModuleDefId :: TraitId (trait_) = def && db . attrs (trait_ . into ()) . has_doc_notable_trait () { traits . push (trait_) ; } } } if traits . is_empty () { None } else { Some (traits . into_iter () . collect ()) } }
    };
}

crate_notable_traits!();