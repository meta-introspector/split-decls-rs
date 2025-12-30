// Generated macro for impl_289 (impl)
macro_rules! Depcrate_stringimpl_289 {
() => {
// Module: crate::string
// Provides: {"impl_289"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > fmt :: Display for StringInner < LenT , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < str as fmt :: Display > :: fmt (self , f) } }
};
}
