// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl EcnCodepoint { # [doc = " Create new object from the given bits"] pub fn from_bits (x : u8) -> Option < Self > { use EcnCodepoint :: * ; Some (match x & 0b11 { 0b10 => Ect0 , 0b01 => Ect1 , 0b11 => Ce , _ => { return None ; } }) } }
};
}
