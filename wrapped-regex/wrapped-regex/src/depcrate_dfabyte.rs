// Generated macro for Byte (struct)
macro_rules! Depcrate_dfaByte {
() => {
// Module: crate::dfa
// Provides: {"Byte"}
// Dependencies: {}
# [doc = " Byte is a u8 in spirit, but a u16 in practice so that we can represent the"] # [doc = " special EOF sentinel value."] # [derive (Copy , Clone , Debug)] struct Byte (u16) ;
};
}
