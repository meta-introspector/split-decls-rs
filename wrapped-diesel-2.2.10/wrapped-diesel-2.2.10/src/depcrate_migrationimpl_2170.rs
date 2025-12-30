// Generated macro for impl_2170 (impl)
macro_rules! Depcrate_migrationimpl_2170 {
() => {
// Module: crate::migration
// Provides: {"impl_2170"}
// Dependencies: {}
impl < 'a , DB > ToSql < Text , DB > for MigrationVersion < 'a > where Cow < 'a , str > : ToSql < Text , DB > , DB : Backend , { fn to_sql < 'b > (& 'b self , out : & mut crate :: serialize :: Output < 'b , '_ , DB > ,) -> crate :: serialize :: Result { self . 0 . to_sql (out) } }
};
}
