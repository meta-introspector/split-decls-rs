// Generated macro for impl_60 (impl)
macro_rules! Depcrate_displayimpl_60 {
() => {
// Module: crate::display
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized > AnsiGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , & 'a S : AsRef < [u8] > , { fn write_inner < W : AnyWrite < Wstr = S > + ? Sized > (& self , w : & mut W) -> Result < () , W :: Error > { match & self . oscontrol { Some (OSControl :: Link { url : u }) => { write ! (w , "\x1B]8;;") ? ; w . write_str (u . as_ref ()) ? ; write ! (w , "\x1B\x5C") ? ; w . write_str (self . string . as_ref ()) ? ; write ! (w , "\x1B]8;;\x1B\x5C") } Some (OSControl :: Title) => { write ! (w , "\x1B]2;") ? ; w . write_str (self . string . as_ref ()) ? ; write ! (w , "\x1B\x5C") } None => w . write_str (self . string . as_ref ()) , } } fn write_to_any < W : AnyWrite < Wstr = S > + ? Sized > (& self , w : & mut W) -> Result < () , W :: Error > { write ! (w , "{}" , self . style . prefix ()) ? ; self . write_inner (w) ? ; write ! (w , "{}" , self . style . suffix ()) } }
};
}
