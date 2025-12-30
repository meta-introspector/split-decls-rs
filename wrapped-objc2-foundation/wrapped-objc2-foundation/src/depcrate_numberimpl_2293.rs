// Generated macro for impl_2293 (impl)
macro_rules! Depcrate_numberimpl_2293 {
() => {
// Module: crate::number
// Provides: {"impl_2293"}
// Dependencies: {}
# [doc = " Beware: This uses the Objective-C method \"isEqualToNumber:\", which has"] # [doc = " different floating point NaN semantics than Rust!"] impl PartialEq for NSNumber { # [doc (alias = "isEqualToNumber:")] fn eq (& self , other : & Self) -> bool { self . isEqualToNumber (other) } }
};
}
