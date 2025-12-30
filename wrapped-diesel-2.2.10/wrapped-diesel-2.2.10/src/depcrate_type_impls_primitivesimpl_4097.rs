// Generated macro for impl_4097 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4097 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4097"}
// Dependencies: {}
impl < 'a , T : ? Sized , ST , DB > ToSql < ST , DB > for Cow < 'a , T > where T : 'a + ToOwned + ToSql < ST , DB > , DB : Backend , Self : fmt :: Debug , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { ToSql :: < ST , DB > :: to_sql (& * * self , out) } }
};
}
