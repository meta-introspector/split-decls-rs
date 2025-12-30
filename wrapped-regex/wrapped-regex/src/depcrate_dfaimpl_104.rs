// Generated macro for impl_104 (impl)
macro_rules! Depcrate_dfaimpl_104 {
() => {
// Module: crate::dfa
// Provides: {"impl_104"}
// Dependencies: {}
impl StateFlags { fn is_match (& self) -> bool { self . 0 & 0b0000000_1 > 0 } fn set_match (& mut self) { self . 0 |= 0b0000000_1 ; } fn is_word (& self) -> bool { self . 0 & 0b000000_1_0 > 0 } fn set_word (& mut self) { self . 0 |= 0b000000_1_0 ; } fn has_empty (& self) -> bool { self . 0 & 0b00000_1_00 > 0 } fn set_empty (& mut self) { self . 0 |= 0b00000_1_00 ; } }
};
}
