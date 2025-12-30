// Generated macro for int (function)
macro_rules! Depcrateint {
() => {
// Module: crate
// Provides: {"int"}
// Dependencies: {}
# [allow (missing_docs)] # [proc_macro] pub fn int (input : TokenStream) -> TokenStream { let mut iter = input . into_iter () ; let min = unwrap_or_return ! (Integer :: try_from_tokens (& mut iter , "minimum value")) ; unwrap_or_return ! (parse_comma (& mut iter)) ; let max = unwrap_or_return ! (Integer :: try_from_tokens (& mut iter , "maximum value")) ; unwrap_or_return ! (Type ::< false >:: from_min_max (& min , & max)) . into_tokens () }
};
}
