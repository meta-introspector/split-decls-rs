// Generated macro for impl_82 (impl)
macro_rules! Depcrate_check_consts_opsimpl_82 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for IntrinsicUnstable { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Unstable { gate : self . feature , gate_already_checked : false , safe_to_expose_on_stable : self . const_stable_indirect , is_function_call : false , } } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnstableIntrinsic { span , name : self . name , feature : self . feature , suggestion : ccx . tcx . crate_level_attribute_injection_span () , }) } }
};
}
