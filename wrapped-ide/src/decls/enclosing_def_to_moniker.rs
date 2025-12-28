macro_rules! deps {
    () => {
        Moniker!();
    };
}

macro_rules! enclosing_def_to_moniker {
    () => {
        deps!();
        fn enclosing_def_to_moniker (db : & RootDatabase , mut def : Definition , from_crate : Crate ,) -> Option < Moniker > { loop { let enclosing_def = def . enclosing_definition (db) ? ; if let Some (enclosing_moniker) = def_to_non_local_moniker (db , enclosing_def , from_crate) { return Some (enclosing_moniker) ; } def = enclosing_def ; } }
    };
}

enclosing_def_to_moniker!();