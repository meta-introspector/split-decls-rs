// Generated macro for bigint_from (macro)
macro_rules! Depcratebigint_from {
() => {
// Module: crate
// Provides: {"bigint_from"}
// Dependencies: {}
macro_rules ! bigint_from { ($ ($ x : ident) *) => ($ (impl From <$ x > for BigInt { # [inline] fn from (x : $ x) -> BigInt { new_bigint_unchecked (& JsValue :: from (x)) } } impl PartialEq <$ x > for BigInt { # [inline] fn eq (& self , other : &$ x) -> bool { JsValue :: from (self) == JsValue :: from (BigInt :: from (* other)) } }) *) }
};
}
