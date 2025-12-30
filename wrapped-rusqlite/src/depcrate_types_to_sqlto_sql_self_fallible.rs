// Generated macro for to_sql_self_fallible (macro)
macro_rules! Depcrate_types_to_sqlto_sql_self_fallible {
() => {
// Module: crate::types::to_sql
// Provides: {"to_sql_self_fallible"}
// Dependencies: {}
# [cfg (feature = "fallible_uint")] macro_rules ! to_sql_self_fallible (($ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: Owned (Value :: Integer (i64 :: try_from (* self) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ?))) } }) ; (non_zero $ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: Owned (Value :: Integer (i64 :: try_from (self . get ()) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ?))) } })) ;
};
}
