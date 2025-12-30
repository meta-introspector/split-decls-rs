// Generated macro for impl_4072 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4072 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4072"}
// Dependencies: {}
impl < T , ST , DB > ToSql < Nullable < ST > , DB > for Option < T > where T : ToSql < ST , DB > , DB : Backend , ST : SqlType < IsNull = is_nullable :: NotNull > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { if let Some (ref value) = * self { value . to_sql (out) } else { Ok (IsNull :: Yes) } } }
};
}
