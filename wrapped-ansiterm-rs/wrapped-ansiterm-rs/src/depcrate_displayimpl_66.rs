// Generated macro for impl_66 (impl)
macro_rules! Depcrate_displayimpl_66 {
() => {
// Module: crate::display
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized + PartialEq > ANSIGenericStrings < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , & 'a S : AsRef < [u8] > { fn write_to_any < W : AnyWrite < Wstr = S > + ? Sized > (& self , w : & mut W) -> Result < () , W :: Error > { use self :: Difference :: * ; let first = match self . 0 . first () { None => return Ok (()) , Some (f) => f , } ; write ! (w , "{}" , first . style . prefix ()) ? ; w . write_str (first . string . as_ref ()) ? ; for window in self . 0 . windows (2) { match Difference :: between (& window [0] . style , & window [1] . style) { ExtraStyles (style) => write ! (w , "{}" , style . prefix ()) ? , Reset => write ! (w , "{}{}" , RESET , window [1] . style . prefix ()) ? , NoDifference => { } , } w . write_str (& window [1] . string) ? ; } if let Some (last) = self . 0 . last () { if ! last . style . is_plain () { write ! (w , "{}" , RESET) ? ; } } Ok (()) } }
};
}
