// Generated macro for from_value (macro)
macro_rules! Depcrate_types_to_sqlfrom_value {
() => {
// Module: crate::types::to_sql
// Provides: {"from_value"}
// Dependencies: {}
macro_rules ! from_value (($ t : ty) => (impl From <$ t > for ToSqlOutput <'_ > { # [inline] fn from (t : $ t) -> Self { ToSqlOutput :: Owned (t . into ()) } }) ; (non_zero $ t : ty) => (impl From <$ t > for ToSqlOutput <'_ > { # [inline] fn from (t : $ t) -> Self { ToSqlOutput :: Owned (t . get () . into ()) } })) ;
};
}
