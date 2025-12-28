macro_rules! deps {
    () => {
        Variant!();
        VariantDef!();
        Field!();
        Union!();
        Struct!();
        Module!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl VariantDef { pub fn fields (self , db : & dyn HirDatabase) -> Vec < Field > { match self { VariantDef :: Struct (it) => it . fields (db) , VariantDef :: Union (it) => it . fields (db) , VariantDef :: Variant (it) => it . fields (db) , } } pub fn module (self , db : & dyn HirDatabase) -> Module { match self { VariantDef :: Struct (it) => it . module (db) , VariantDef :: Union (it) => it . module (db) , VariantDef :: Variant (it) => it . module (db) , } } pub fn name (& self , db : & dyn HirDatabase) -> Name { match self { VariantDef :: Struct (s) => s . name (db) , VariantDef :: Union (u) => u . name (db) , VariantDef :: Variant (e) => e . name (db) , } } }
    };
}

impl_279!();