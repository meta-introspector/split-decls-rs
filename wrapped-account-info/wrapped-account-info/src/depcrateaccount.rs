// Generated macro for Account (trait)
macro_rules! DepcrateAccount {
() => {
// Module: crate
// Provides: {"Account"}
// Dependencies: {}
# [doc = " Provides information required to construct an `AccountInfo`, used in"] # [doc = " conversion implementations."] pub trait Account { fn get (& mut self) -> (& mut u64 , & mut [u8] , & Address , bool) ; }
};
}
