macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! crate_name {
    () => {
        deps!();
        fn crate_name (db : & RootDatabase , krate : Crate) -> Symbol { krate . extra_data (db) . display_name . as_deref () . cloned () . unwrap_or_else (| | Symbol :: integer (salsa :: plumbing :: AsId :: as_id (& krate) . index () as usize)) }
    };
}

crate_name!();