// Generated macro for Ring (enum)
macro_rules! DepcrateRing {
() => {
// Module: crate
// Provides: {"Ring"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u8)] # [doc = " x86 Protection levels"] # [doc = ""] # [doc = " # Note"] # [doc = " This should not contain values larger than 2 bits, otherwise"] # [doc = " segment descriptor code needs to be adjusted accordingly."] pub enum Ring { Ring0 = 0b00 , Ring1 = 0b01 , Ring2 = 0b10 , Ring3 = 0b11 , }
};
}
