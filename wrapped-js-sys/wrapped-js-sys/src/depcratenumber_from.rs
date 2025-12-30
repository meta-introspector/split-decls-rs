// Generated macro for number_from (macro)
macro_rules! Depcratenumber_from {
() => {
// Module: crate
// Provides: {"number_from"}
// Dependencies: {}
macro_rules ! number_from { ($ ($ x : ident) *) => ($ (impl From <$ x > for Number { # [inline] fn from (x : $ x) -> Number { Number :: unchecked_from_js (JsValue :: from (x)) } } impl PartialEq <$ x > for Number { # [inline] fn eq (& self , other : &$ x) -> bool { self . value_of () == f64 :: from (* other) } }) *) }
};
}
