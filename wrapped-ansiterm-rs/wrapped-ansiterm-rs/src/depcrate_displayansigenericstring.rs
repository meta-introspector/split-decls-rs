// Generated macro for ANSIGenericString (struct)
macro_rules! Depcrate_displayANSIGenericString {
() => {
// Module: crate::display
// Provides: {"ANSIGenericString"}
// Dependencies: {}
# [doc = " An `ANSIGenericString` includes a generic string type and a `Style` to"] # [doc = " display that string.  `ANSIString` and `ANSIByteString` are aliases for"] # [doc = " this type on `str` and `\\[u8]`, respectively."] # [derive (PartialEq , Debug)] pub struct ANSIGenericString < 'a , S : 'a + ToOwned + ? Sized > where < S as ToOwned > :: Owned : fmt :: Debug { style : Style , string : Cow < 'a , S > , }
};
}
