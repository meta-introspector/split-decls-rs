// Generated macro for get_ty (function)
macro_rules! Depcrate_parserget_ty {
() => {
// Module: crate::parser
// Provides: {"get_ty"}
// Dependencies: {}
fn get_ty (mut ty : & syn :: Type) -> & syn :: Type { while let syn :: Type :: Group (g) = ty { ty = & g . elem ; } ty }
};
}
