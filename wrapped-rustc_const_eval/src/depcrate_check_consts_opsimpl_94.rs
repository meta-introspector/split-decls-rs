// Generated macro for impl_94 (impl)
macro_rules! Depcrate_check_consts_opsimpl_94 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for EscapingMutBorrow { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Forbidden } fn importance (& self) -> DiagImportance { DiagImportance :: Secondary } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: MutableBorrowEscaping { span , kind : ccx . const_kind () }) } }
};
}
