// Generated macro for opt_int (function)
macro_rules! Depcrateopt_int {
() => {
// Module: crate
// Provides: {"opt_int"}
// Dependencies: {}
# [allow (missing_docs)] # [proc_macro] pub fn opt_int (input : TokenStream) -> TokenStream { let mut iter = input . into_iter () ; let min = unwrap_or_return ! (Integer :: try_from_tokens (& mut iter , "minimum value")) ; unwrap_or_return ! (parse_comma (& mut iter)) ; let max = unwrap_or_return ! (Integer :: try_from_tokens (& mut iter , "maximum value")) ; unwrap_or_return ! (Type ::< true >:: from_min_max (& min , & max)) . into_tokens () }
};
}
