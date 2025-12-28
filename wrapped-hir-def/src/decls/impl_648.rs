macro_rules! deps {
    () => {
        DefDatabase!();
        ExpressionStoreSourceMap!();
        VariantFields!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl EnumVariantId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
    };
}

impl_648!();