// Generated macro for generate_per_field_offsets (function)
macro_rules! Depcrate_utilsgenerate_per_field_offsets {
() => {
// Module: crate::utils
// Provides: {"generate_per_field_offsets"}
// Dependencies: {}
# [doc = " Given an iterator over ULE or AsULE struct fields, returns code that calculates field sizes and generates a line"] # [doc = " of code per field based on the per_field_code function (whose parameters are the field, the identifier of the const"] # [doc = " for the previous offset, the identifier for the const for the next offset, and the field index)"] pub (crate) fn generate_per_field_offsets < 'a > (fields : & [FieldInfo < 'a >] , fields_are_asule : bool , mut per_field_code : impl FnMut (& FieldInfo < 'a > , & Ident , & Ident) -> TokenStream2 ,) -> (TokenStream2 , syn :: Ident) { let mut prev_offset_ident = Ident :: new ("ZERO" , Span :: call_site ()) ; let mut code = quote ! (const ZERO : usize = 0 ;) ; for (i , field_info) in fields . iter () . enumerate () { let field = & field_info . field ; let ty = & field . ty ; let ty = if fields_are_asule { quote ! (<# ty as zerovec :: ule :: AsULE >:: ULE) } else { quote ! (# ty) } ; let new_offset_ident = suffixed_ident ("OFFSET" , i , field . span ()) ; let size_ident = suffixed_ident ("SIZE" , i , field . span ()) ; let pf_code = per_field_code (field_info , & prev_offset_ident , & size_ident) ; code = quote ! { # code ; const # size_ident : usize = :: core :: mem :: size_of ::<# ty > () ; const # new_offset_ident : usize = # prev_offset_ident + # size_ident ; # pf_code ; } ; prev_offset_ident = new_offset_ident ; } (code , prev_offset_ident) }
};
}
