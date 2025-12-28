macro_rules! deps {
    () => {
        HasSource!();
        VariantDef!();
        Struct!();
        Variant!();
        Union!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl HasSource for VariantDef { type Ast = ast :: VariantDef ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self { VariantDef :: Struct (s) => Some (s . source (db) ? . map (ast :: VariantDef :: Struct)) , VariantDef :: Union (u) => Some (u . source (db) ? . map (ast :: VariantDef :: Union)) , VariantDef :: Variant (v) => Some (v . source (db) ? . map (ast :: VariantDef :: Variant)) , } } }
    };
}

impl_50!()