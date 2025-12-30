// Generated macro for bigint_from_big (macro)
macro_rules! Depcratebigint_from_big {
() => {
// Module: crate
// Provides: {"bigint_from_big"}
// Dependencies: {}
macro_rules ! bigint_from_big { ($ ($ x : ident) *) => ($ (impl From <$ x > for BigInt { # [inline] fn from (x : $ x) -> BigInt { JsValue :: from (x) . unchecked_into () } } impl PartialEq <$ x > for BigInt { # [inline] fn eq (& self , other : &$ x) -> bool { self == & BigInt :: from (* other) } } impl TryFrom < BigInt > for $ x { type Error = BigInt ; # [inline] fn try_from (x : BigInt) -> Result < Self , BigInt > { Self :: try_from (JsValue :: from (x)) . map_err (JsCast :: unchecked_into) } }) *) }
};
}
