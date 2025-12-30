// Generated macro for PtrauthKey (enum)
macro_rules! Depcrate_machoPtrauthKey {
() => {
// Module: crate::macho
// Provides: {"PtrauthKey"}
// Dependencies: {}
# [doc = " The key used to sign a pointer for authentication."] # [doc = ""] # [doc = " The variant values correspond to the values used in the"] # [doc = " `ptrauth_key` enum in `ptrauth.h`."] # [repr (u8)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum PtrauthKey { # [doc = " Instruction key A."] IA = 0 , # [doc = " Instruction key B."] IB = 1 , # [doc = " Data key A."] DA = 2 , # [doc = " Data key B."] DB = 3 , }
};
}
