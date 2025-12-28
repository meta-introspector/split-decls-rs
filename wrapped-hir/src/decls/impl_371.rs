macro_rules! deps {
    () => {
        GenericDef!();
        LifetimeParam!();
        Module!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl LifetimeParam { pub fn name (self , db : & dyn HirDatabase) -> Name { let params = db . generic_params (self . id . parent) ; params [self . id . local_id] . name . clone () } pub fn module (self , db : & dyn HirDatabase) -> Module { self . id . parent . module (db) . into () } pub fn parent (self , _db : & dyn HirDatabase) -> GenericDef { self . id . parent . into () } }
    };
}

impl_371!();