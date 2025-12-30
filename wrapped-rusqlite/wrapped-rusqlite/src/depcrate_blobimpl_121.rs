// Generated macro for impl_121 (impl)
macro_rules! Depcrate_blobimpl_121 {
() => {
// Module: crate::blob
// Provides: {"impl_121"}
// Dependencies: {}
impl ToSql for ZeroBlob { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let Self (length) = * self ; Ok (ToSqlOutput :: ZeroBlob (length)) } }
};
}
