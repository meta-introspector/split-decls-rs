// Generated macro for impl_378 (impl)
macro_rules! Depcrateimpl_378 {
() => {
// Module: crate
// Provides: {"impl_378"}
// Dependencies: {}
impl DemangleOptions { # [doc = " Construct a new `DemangleOptions` with the default values."] pub fn new () -> Self { Default :: default () } # [doc = " Do not display function arguments."] pub fn no_params (mut self) -> Self { self . no_params = true ; self } # [doc = " Do not display the function return type."] pub fn no_return_type (mut self) -> Self { self . no_return_type = true ; self } # [doc = " Hide type annotations in template value parameters."] # [doc = " These are not needed to distinguish template instances"] # [doc = " so this can make it easier to match user-provided"] # [doc = " template instance names."] pub fn hide_expression_literal_types (mut self) -> Self { self . hide_expression_literal_types = true ; self } # [doc = " Set the limit on recursion depth during the demangling phase. A low"] # [doc = " limit will cause valid symbols to be rejected, but a high limit may"] # [doc = " allow pathological symbols to overflow the stack during demangling."] # [doc = " The default value is 128."] pub fn recursion_limit (mut self , limit : u32) -> Self { self . recursion_limit = Some (NonZeroU32 :: new (limit) . expect ("Recursion limit must be > 0")) ; self } }
};
}
