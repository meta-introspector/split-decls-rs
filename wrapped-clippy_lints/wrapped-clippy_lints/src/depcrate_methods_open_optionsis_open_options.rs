// Generated macro for is_open_options (function)
macro_rules! Depcrate_methods_open_optionsis_open_options {
() => {
// Module: crate::methods::open_options
// Provides: {"is_open_options"}
// Dependencies: {}
fn is_open_options (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_diag_item (cx , sym :: FsOpenOptions) || paths :: TOKIO_IO_OPEN_OPTIONS . matches_ty (cx , ty) }
};
}
