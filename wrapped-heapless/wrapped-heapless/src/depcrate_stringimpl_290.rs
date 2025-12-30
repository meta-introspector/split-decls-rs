// Generated macro for impl_290 (impl)
macro_rules! Depcrate_stringimpl_290 {
() => {
// Module: crate::string
// Provides: {"impl_290"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > hash :: Hash for StringInner < LenT , S > { # [inline] fn hash < H : hash :: Hasher > (& self , hasher : & mut H) { < str as hash :: Hash > :: hash (self , hasher) ; } }
};
}
