// Generated macro for impl_288 (impl)
macro_rules! Depcrate_stringimpl_288 {
() => {
// Module: crate::string
// Provides: {"impl_288"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > fmt :: Debug for StringInner < LenT , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < str as fmt :: Debug > :: fmt (self , f) } }
};
}
