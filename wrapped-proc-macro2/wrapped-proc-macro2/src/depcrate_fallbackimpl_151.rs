// Generated macro for impl_151 (impl)
macro_rules! Depcrate_fallbackimpl_151 {
() => {
// Module: crate::fallback
// Provides: {"impl_151"}
// Dependencies: {}
impl Display for Group { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (open , close) = match self . delimiter { Delimiter :: Parenthesis => ("(" , ")") , Delimiter :: Brace => ("{ " , "}") , Delimiter :: Bracket => ("[" , "]") , Delimiter :: None => ("" , "") , } ; f . write_str (open) ? ; Display :: fmt (& self . stream , f) ? ; if self . delimiter == Delimiter :: Brace && ! self . stream . inner . is_empty () { f . write_str (" ") ? ; } f . write_str (close) ? ; Ok (()) } }
};
}
