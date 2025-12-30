// Generated macro for impl_914 (impl)
macro_rules! Depcrateimpl_914 {
() => {
// Module: crate
// Provides: {"impl_914"}
// Dependencies: {}
impl From < CallableDefId > for ModuleDefId { fn from (def : CallableDefId) -> ModuleDefId { match def { CallableDefId :: FunctionId (f) => ModuleDefId :: FunctionId (f) , CallableDefId :: StructId (s) => ModuleDefId :: AdtId (AdtId :: StructId (s)) , CallableDefId :: EnumVariantId (e) => ModuleDefId :: EnumVariantId (e) , } } }
};
}
