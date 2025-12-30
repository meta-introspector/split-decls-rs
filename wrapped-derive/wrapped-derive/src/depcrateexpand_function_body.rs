// Generated macro for expand_function_body (function)
macro_rules! Depcrateexpand_function_body {
() => {
// Module: crate
// Provides: {"expand_function_body"}
// Dependencies: {}
fn expand_function_body (function : Function) -> TokenStream2 { let Function { attrs , vis , constness , asyncness , unsafety , abi , fn_token , ident , generics , paren_token , arg , colon_token , from_type , arrow_token , to_type , semi_token , } = function ; let args = quote_spanned ! { paren_token . span => (# arg # colon_token # from_type) } ; let allow_unused_unsafe = if unsafety . is_some () { Some (quote ! (# [allow (unused_unsafe)])) } else { None } ; let mut inline_attr = Some (quote ! (# [inline])) ; for attr in & attrs { if attr . path () . is_ident ("inline") { inline_attr = None ; break ; } } let macro_generated_unsafe = quote ! (unsafe) ; quote_spanned ! { semi_token . span => # (# attrs) * # inline_attr # vis # constness # asyncness # unsafety # abi # fn_token # ident # generics # args # arrow_token # to_type { let _ = || { :: ref_cast ::# private :: ref_cast_custom ::<# from_type , # to_type > (# arg) ; } ; let _ = :: ref_cast ::# private :: CurrentCrate ::<# from_type , # to_type > { } ; # allow_unused_unsafe # [allow (clippy :: transmute_ptr_to_ptr)] # macro_generated_unsafe { :: ref_cast ::# private :: transmute ::<# from_type , # to_type > (# arg) } } } }
};
}
