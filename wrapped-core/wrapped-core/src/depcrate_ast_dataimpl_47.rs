// Generated macro for impl_47 (impl)
macro_rules! Depcrate_ast_dataimpl_47 {
() => {
// Module: crate::ast::data
// Provides: {"impl_47"}
// Dependencies: {}
impl From < & syn :: Fields > for Style { fn from (vd : & syn :: Fields) -> Self { match * vd { syn :: Fields :: Named (_) => Style :: Struct , syn :: Fields :: Unnamed (_) => Style :: Tuple , syn :: Fields :: Unit => Style :: Unit , } } }
};
}
