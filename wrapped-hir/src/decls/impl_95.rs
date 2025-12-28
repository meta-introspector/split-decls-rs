macro_rules! deps {
    () => {
        Module!();
        ExternBlock!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl ExternBlock { pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . module (db) } } }
    };
}

impl_95!()