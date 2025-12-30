// Generated macro for impl_200 (impl)
macro_rules! Depcrate_deserializeimpl_200 {
() => {
// Module: crate::deserialize
// Provides: {"impl_200"}
// Dependencies: {}
impl < T , ST , DB > FromSqlRow < ST , DB > for T where T : Queryable < ST , DB > , ST : SqlTypeOrSelectable , DB : Backend , T :: Row : FromStaticSqlRow < ST , DB > , { # [inline (always)] fn build_from_row < 'a > (row : & impl Row < 'a , DB >) -> Result < Self > { let row = < T :: Row as FromStaticSqlRow < ST , DB > > :: build_from_row (row) ? ; T :: build (row) } }
};
}
