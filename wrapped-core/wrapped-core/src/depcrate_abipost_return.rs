// Generated macro for post_return (function)
macro_rules! Depcrate_abipost_return {
() => {
// Module: crate::abi
// Provides: {"post_return"}
// Dependencies: {}
# [doc = " Used in a similar manner as the `Interface::call` function except is"] # [doc = " used to generate the `post-return` callback for `func`."] # [doc = ""] # [doc = " This is only intended to be used in guest generators for exported"] # [doc = " functions and will primarily generate `GuestDeallocate*` instructions,"] # [doc = " plus others used as input to those instructions."] pub fn post_return (resolve : & Resolve , func : & Function , bindgen : & mut impl Bindgen) { Generator :: new (resolve , bindgen) . post_return (func) ; }
};
}
