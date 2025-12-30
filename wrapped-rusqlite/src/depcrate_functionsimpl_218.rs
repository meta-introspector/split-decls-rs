// Generated macro for impl_218 (impl)
macro_rules! Depcrate_functionsimpl_218 {
() => {
// Module: crate::functions
// Provides: {"impl_218"}
// Dependencies: {}
impl < T : ToSql > SqlFnOutput for T { # [inline] fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > { ToSql :: to_sql (self) . map (| o | (o , None)) } }
};
}
