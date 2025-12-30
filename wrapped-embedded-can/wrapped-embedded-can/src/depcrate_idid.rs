// Generated macro for Id (enum)
macro_rules! Depcrate_idId {
() => {
// Module: crate::id
// Provides: {"Id"}
// Dependencies: {}
# [doc = " A CAN Identifier (standard or extended)."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum Id { # [doc = " Standard 11-bit Identifier (`0..=0x7FF`)."] Standard (StandardId) , # [doc = " Extended 29-bit Identifier (`0..=0x1FFF_FFFF`)."] Extended (ExtendedId) , }
};
}
