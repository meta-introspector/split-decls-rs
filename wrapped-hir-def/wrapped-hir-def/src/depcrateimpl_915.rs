// Generated macro for impl_915 (impl)
macro_rules! Depcrateimpl_915 {
() => {
// Module: crate
// Provides: {"impl_915"}
// Dependencies: {}
impl CallableDefId { pub fn krate (self , db : & dyn DefDatabase) -> Crate { match self { CallableDefId :: FunctionId (f) => f . krate (db) , CallableDefId :: StructId (s) => s . krate (db) , CallableDefId :: EnumVariantId (e) => e . krate (db) , } } }
};
}
