// Generated macro for expand_ref_cast_custom (function)
macro_rules! Depcrateexpand_ref_cast_custom {
() => {
// Module: crate
// Provides: {"expand_ref_cast_custom"}
// Dependencies: {}
fn expand_ref_cast_custom (input : & DeriveInput) -> Result < TokenStream2 > { check_repr (input) ? ; let vis = & input . vis ; let name = & input . ident ; let (impl_generics , ty_generics , where_clause) = input . generics . split_for_impl () ; let fields = fields (input) ? ; let from = only_field_ty (fields) ? ; let trivial = trivial_fields (fields) ? ; let private2 = private ; let assert_trivial_fields = if ! trivial . is_empty () { Some (quote ! { fn __static_assert () { if false { # (:: ref_cast ::# private2 :: assert_trivial ::<# trivial > () ;) * } } }) } else { None } ; Ok (quote ! { const _ : () = { # [non_exhaustive] # vis struct RefCastCurrentCrate { } unsafe impl # impl_generics :: ref_cast ::# private :: RefCastCustom <# from > for # name # ty_generics # where_clause { type CurrentCrate = RefCastCurrentCrate ; # assert_trivial_fields } } ; }) }
};
}
