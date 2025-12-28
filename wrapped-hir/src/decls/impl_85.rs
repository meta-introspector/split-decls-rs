macro_rules! deps {
    () => {
        SemanticsImpl!();
        Semantics!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < DB : HirDatabase > Semantics < '_ , DB > { # [doc = " Creates an instance that's strongly coupled to its underlying database type."] pub fn new (db : & DB) -> Semantics < '_ , DB > { let impl_ = SemanticsImpl :: new (db) ; Semantics { db , imp : impl_ } } }
    };
}

impl_85!()