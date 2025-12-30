// Generated macro for ANSIGenericStrings (struct)
macro_rules! Depcrate_displayANSIGenericStrings {
() => {
// Module: crate::display
// Provides: {"ANSIGenericStrings"}
// Dependencies: {}
# [doc = " A set of `ANSIGenericString`s collected together, in order to be"] # [doc = " written with a minimum of control characters."] # [derive (Debug , PartialEq)] pub struct ANSIGenericStrings < 'a , S : 'a + ToOwned + ? Sized > (pub & 'a [ANSIGenericString < 'a , S >]) where < S as ToOwned > :: Owned : fmt :: Debug , S : PartialEq ;
};
}
