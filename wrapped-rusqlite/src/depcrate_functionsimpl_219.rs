// Generated macro for impl_219 (impl)
macro_rules! Depcrate_functionsimpl_219 {
() => {
// Module: crate::functions
// Provides: {"impl_219"}
// Dependencies: {}
impl < T : ToSql > SqlFnOutput for (T , SubType) { fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > { ToSql :: to_sql (& self . 0) . map (| o | (o , self . 1)) } }
};
}
