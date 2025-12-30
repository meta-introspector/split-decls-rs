// Generated macro for impl_86 (impl)
macro_rules! Depcrate_check_consts_opsimpl_86 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for HeapAllocation { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedHeapAllocations { span , kind : ccx . const_kind () , teach : ccx . tcx . sess . teach (E0010) , }) } }
};
}
