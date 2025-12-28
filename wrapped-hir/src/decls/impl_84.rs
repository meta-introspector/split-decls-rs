macro_rules! deps {
    () => {
        SemanticsImpl!();
        Semantics!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Semantics < '_ , dyn HirDatabase > { # [doc = " Creates an instance that's weakly coupled to its underlying database type."] pub fn new_dyn (db : & '_ dyn HirDatabase) -> Semantics < '_ , dyn HirDatabase > { let impl_ = SemanticsImpl :: new (db) ; Semantics { db , imp : impl_ } } }
    };
}

impl_84!()