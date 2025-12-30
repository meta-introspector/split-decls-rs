// Generated macro for validate_parameter_attrs (function)
macro_rules! Depcrate_property_test_validatevalidate_parameter_attrs {
() => {
// Module: crate::property_test::validate
// Provides: {"validate_parameter_attrs"}
// Dependencies: {}
# [doc = " Make sure we only have `#[strategy = <expr>]` attributes on function parameters"] fn validate_parameter_attrs (f : & mut ItemFn) -> Result < () , TokenStream > { let mut error = quote :: quote ! { } ; for param in & mut f . sig . inputs { let FnArg :: Typed (pat_ty) = param else { unreachable ! ("should be impossible due to `all_args_non_self`") ; } ; for attr in pat_ty . attrs . iter () . filter (| a | ! is_strategy (a)) { error . extend (quote_spanned ! { attr . span () => compile_error ! ("only `#[strategy = <expr>]` attributes are allowed here") ; }) ; } let mut first_strategy_seen = false ; let mut final_attrs = Vec :: with_capacity (pat_ty . attrs . len ()) ; let old_attrs = std :: mem :: take (& mut pat_ty . attrs) ; for attr in old_attrs . into_iter () . filter (is_strategy) { match attr . meta { Meta :: NameValue (_) => { if first_strategy_seen { let pat = pat_ty . pat . clone () . into_token_stream () . to_string () ; let message = format ! ("{pat} has duplicate `#[strategy = ...] attribute`") ; error . extend (quote_spanned ! { attr . span () => compile_error ! (# message) ; }) ; } else { final_attrs . push (attr) ; first_strategy_seen = true ; } } _ => { error . extend (quote_spanned ! { attr . meta . span () => compile_error ! ("`strategy` attributes must have the form `#[strategy = <expr>]`") ; }) ; final_attrs . push (attr) ; } } } pat_ty . attrs = final_attrs ; } if error . is_empty () { Ok (()) } else { Err (error) } }
};
}
