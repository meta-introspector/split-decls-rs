// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl FnSig { fn update_for_func (& mut self , func : & Function) { if let FunctionKind :: Method (_) | FunctionKind :: AsyncMethod (_) = & func . kind { self . self_arg = Some ("&self" . into ()) ; self . self_is_first_param = true ; } } }
};
}
