// Generated macro for impl_872 (impl)
macro_rules! Depcrate_sharedimpl_872 {
() => {
// Module: crate::shared
// Provides: {"impl_872"}
// Dependencies: {}
impl EcnCodepoint { # [doc = " Create new object from the given bits"] pub fn from_bits (x : u8) -> Option < Self > { use EcnCodepoint :: * ; Some (match x & 0b11 { 0b10 => Ect0 , 0b01 => Ect1 , 0b11 => Ce , _ => { return None ; } }) } # [doc = " Returns whether the codepoint is a CE, signalling that congestion was experienced"] pub fn is_ce (self) -> bool { matches ! (self , Self :: Ce) } }
};
}
