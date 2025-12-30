// Generated macro for impl_823 (impl)
macro_rules! Depcrateimpl_823 {
() => {
// Module: crate
// Provides: {"impl_823"}
// Dependencies: {}
impl StructId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
};
}
