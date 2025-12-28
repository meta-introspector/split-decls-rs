macro_rules! deps {
    () => {
        ExpressionStoreSourceMap!();
        VariantFields!();
        DefDatabase!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl UnionId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
    };
}

impl_622!();