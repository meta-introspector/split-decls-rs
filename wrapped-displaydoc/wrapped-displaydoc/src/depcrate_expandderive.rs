// Generated macro for derive (function)
macro_rules! Depcrate_expandderive {
() => {
// Module: crate::expand
// Provides: {"derive"}
// Dependencies: {}
pub (crate) fn derive (input : & DeriveInput) -> Result < TokenStream > { let impls = match & input . data { Data :: Struct (data) => impl_struct (input , data) , Data :: Enum (data) => impl_enum (input , data) , Data :: Union (_) => Err (Error :: new_spanned (input , "Unions are not supported")) , } ? ; let helpers = specialization () ; Ok (quote ! { # [allow (non_upper_case_globals , unused_attributes , unused_qualifications)] const _ : () = { # helpers # impls } ; }) }
};
}
