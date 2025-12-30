// Generated macro for impl_63 (impl)
macro_rules! Depcrate_displayimpl_63 {
() => {
// Module: crate::display
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized + PartialEq > AnsiGenericStrings < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , & 'a S : AsRef < [u8] > , { fn write_to_any < W : AnyWrite < Wstr = S > + ? Sized > (& self , w : & mut W) -> Result < () , W :: Error > { use self :: Difference :: * ; let first = match self . 0 . first () { None => return Ok (()) , Some (f) => f , } ; write ! (w , "{}" , first . style . prefix ()) ? ; first . write_inner (w) ? ; for window in self . 0 . windows (2) { match Difference :: between (& window [0] . style , & window [1] . style) { ExtraStyles (style) => write ! (w , "{}" , style . prefix ()) ? , Reset => write ! (w , "{}{}" , RESET , window [1] . style . prefix ()) ? , Empty => { } } window [1] . write_inner (w) ? ; } if let Some (last) = self . 0 . last () { if ! last . style . is_plain () { write ! (w , "{}" , RESET) ? ; } } Ok (()) } }
};
}
