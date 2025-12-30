// Generated macro for try_expand (function)
macro_rules! Depcrate_expandtry_expand {
() => {
// Module: crate::expand
// Provides: {"try_expand"}
// Dependencies: {}
fn try_expand (input : & DeriveInput) -> Result < TokenStream > { let input = Input :: from_syn (input) ? ; input . validate () ? ; Ok (match input { Input :: Struct (input) => impl_struct (input) , Input :: Enum (input) => impl_enum (input) , }) }
};
}
