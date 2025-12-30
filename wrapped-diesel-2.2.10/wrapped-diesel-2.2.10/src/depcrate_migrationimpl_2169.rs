// Generated macro for impl_2169 (impl)
macro_rules! Depcrate_migrationimpl_2169 {
() => {
// Module: crate::migration
// Provides: {"impl_2169"}
// Dependencies: {}
impl < DB > FromSql < Text , DB > for MigrationVersion < '_ > where String : FromSql < Text , DB > , DB : Backend , { fn from_sql (bytes : DB :: RawValue < '_ >) -> crate :: deserialize :: Result < Self > { let s = String :: from_sql (bytes) ? ; Ok (Self (Cow :: Owned (s))) } }
};
}
