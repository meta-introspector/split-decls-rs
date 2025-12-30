// Generated macro for Padding (struct)
macro_rules! Depcrate_rsaPadding {
() => {
// Module: crate::rsa
// Provides: {"Padding"}
// Dependencies: {}
# [doc = " Type of encryption padding to use."] # [doc = ""] # [doc = " Random length padding is primarily used to prevent attackers from"] # [doc = " predicting or knowing the exact length of a plaintext message that"] # [doc = " can possibly lead to breaking encryption."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct Padding (c_int) ;
};
}
