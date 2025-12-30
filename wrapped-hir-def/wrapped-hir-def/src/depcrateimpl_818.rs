// Generated macro for impl_818 (impl)
macro_rules! Depcrateimpl_818 {
() => {
// Module: crate
// Provides: {"impl_818"}
// Dependencies: {}
impl StructId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
};
}
