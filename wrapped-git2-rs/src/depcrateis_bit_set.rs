// Generated macro for is_bit_set (macro)
macro_rules! Depcrateis_bit_set {
() => {
// Module: crate
// Provides: {"is_bit_set"}
// Dependencies: {}
macro_rules ! is_bit_set { ($ name : ident , $ flag : expr) => { # [allow (missing_docs)] pub fn $ name (& self) -> bool { self . intersects ($ flag) } } ; }
};
}
