// Generated macro for CustomizedInit (trait)
macro_rules! DepcrateCustomizedInit {
() => {
// Module: crate
// Provides: {"CustomizedInit"}
// Dependencies: {}
# [doc = " Trait for hash functions with customization string for domain separation."] pub trait CustomizedInit : Sized { # [doc = " Create new hasher instance with the given customization string."] fn new_customized (customization : & [u8]) -> Self ; }
};
}
