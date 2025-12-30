// Generated macro for impl_78 (impl)
macro_rules! Depcrate_check_consts_opsimpl_78 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for CallUnstable { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Unstable { gate : self . feature , gate_already_checked : self . feature_enabled , safe_to_expose_on_stable : self . safe_to_expose_on_stable , is_function_call : self . is_function_call , } } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { assert ! (! self . feature_enabled) ; let mut err = if self . is_function_call { ccx . dcx () . create_err (errors :: UnstableConstFn { span , def_path : ccx . tcx . def_path_str (self . def_id) , }) } else { ccx . dcx () . create_err (errors :: UnstableConstTrait { span , def_path : ccx . tcx . def_path_str (self . def_id) , }) } ; ccx . tcx . disabled_nightly_features (& mut err , [(String :: new () , self . feature)]) ; err } }
};
}
