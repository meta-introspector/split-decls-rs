// Generated macro for generate_default_value (function)
macro_rules! Depcrate_utilsgenerate_default_value {
() => {
// Module: crate::utils
// Provides: {"generate_default_value"}
// Dependencies: {}
fn generate_default_value (lit : & Lit) -> GeneratorResult < TokenStream > { match lit { Lit :: Str (value) => { let value = value . value () ; Ok (quote ! ({ :: std :: borrow :: ToOwned :: to_owned (# value) })) } Lit :: Int (value) => { let value = value . base10_parse :: < i32 > () ? ; Ok (quote ! ({ :: std :: convert :: TryInto :: try_into (# value) . unwrap_or_default () })) } Lit :: Float (value) => { let value = value . base10_parse :: < f64 > () ? ; Ok (quote ! ({ :: std :: convert :: TryInto :: try_into (# value) . unwrap_or_default () })) } Lit :: Bool (value) => { let value = value . value ; Ok (quote ! ({ # value })) } _ => Err (Error :: new_spanned (lit , "The default value type only be string, integer, float and boolean, other types should use default_with" ,) . into ()) , } }
};
}
