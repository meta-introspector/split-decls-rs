// Generated macro for derive_ex_derive (function)
macro_rules! Depcratederive_ex_derive {
() => {
// Module: crate
// Provides: {"derive_ex_derive"}
// Dependencies: {}
# [doc = " Use attribute macro [`macro@derive_ex`] as derive macro."] # [doc = ""] # [doc = " [`macro@derive_ex`], being an attribute macro designed to mimic the functionality of the derive macro,"] # [doc = " may cause rust-analyzer's assistance to not work correctly in certain cases."] # [doc = ""] # [doc = " Adding `#[derive(Ex)]` to an item with `#[derive_ex]` will allow rust-analyzer's assistance to work correctly."] # [doc = ""] # [doc = " In the example below, without `#[derive(Ex)]`,"] # [doc = " the jump from `value: String` to the definition of `String` is not possible,"] # [doc = " but with `#[derive(Ex)]`, it is possible."] # [doc = ""] # [doc = " ```"] # [doc = " use derive_ex::Ex;"] # [doc = ""] # [doc = " #[derive(Ex)]"] # [doc = " #[derive_ex(Eq, PartialEq)]"] # [doc = " struct X {"] # [doc = "     #[eq(key = $.len())]"] # [doc = "     value: String,"] # [doc = " }"] # [doc = " ```"] # [proc_macro_derive (Ex , attributes (derive_ex , ord , partial_ord , eq , partial_eq , hash , debug , default))] pub fn derive_ex_derive (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input : TokenStream = input . into () ; match item_type :: build_derive (input) { Ok (s) => s , Err (e) => e . to_compile_error () , } . into () }
};
}
