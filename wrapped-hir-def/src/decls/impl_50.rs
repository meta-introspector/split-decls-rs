macro_rules! impl_50 {
    () => {
        impl UnionId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
    };
}

impl_50!()