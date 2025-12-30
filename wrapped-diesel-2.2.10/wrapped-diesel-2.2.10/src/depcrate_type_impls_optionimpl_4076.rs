// Generated macro for impl_4076 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4076 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4076"}
// Dependencies: {}
impl < ST , T , DB > Queryable < ST , DB > for Option < T > where ST : SingleValue < IsNull = is_nullable :: IsNullable > , DB : Backend , Self : FromSql < ST , DB > , { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
