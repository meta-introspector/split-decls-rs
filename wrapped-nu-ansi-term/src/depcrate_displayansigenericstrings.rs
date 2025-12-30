// Generated macro for AnsiGenericStrings (struct)
macro_rules! Depcrate_displayAnsiGenericStrings {
() => {
// Module: crate::display
// Provides: {"AnsiGenericStrings"}
// Dependencies: {}
# [doc = " A set of `AnsiGenericStrings`s collected together, in order to be"] # [doc = " written with a minimum of control characters."] # [derive (Debug , Eq , PartialEq)] pub struct AnsiGenericStrings < 'a , S : 'a + ToOwned + ? Sized > (pub & 'a [AnsiGenericString < 'a , S >]) where < S as ToOwned > :: Owned : fmt :: Debug , S : PartialEq ;
};
}
