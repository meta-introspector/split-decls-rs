// Generated macro for impl_63 (impl)
macro_rules! Depcrate_displayimpl_63 {
() => {
// Module: crate::display
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized > ANSIGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , & 'a S : AsRef < [u8] > { fn write_to_any < W : AnyWrite < Wstr = S > + ? Sized > (& self , w : & mut W) -> Result < () , W :: Error > { write ! (w , "{}" , self . style . prefix ()) ? ; w . write_str (self . string . as_ref ()) ? ; write ! (w , "{}" , self . style . suffix ()) } }
};
}
