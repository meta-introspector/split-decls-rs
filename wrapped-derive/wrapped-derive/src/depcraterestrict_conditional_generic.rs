// Generated macro for restrict_conditional_generic (function)
macro_rules! Depcraterestrict_conditional_generic {
() => {
// Module: crate
// Provides: {"restrict_conditional_generic"}
// Dependencies: {}
# [cfg (feature = "conditional_deserialization")] fn restrict_conditional_generic (impl_generics : ImplGenerics , ty_generics : TypeGenerics , deserializable : bool ,) -> (TokenStream2 , TokenStream2) { let impl_generics = quote ! { # impl_generics } . to_string () . replace ('\n' , " ") . replace (" const IS_DESERIALIZABLE : bool " , "") . replace ("<>" , "") . parse () . unwrap () ; let deserializable_string = if deserializable { "true" } else { "false" } ; let ty_generics = quote ! { # ty_generics } . to_string () . replace ("IS_DESERIALIZABLE" , deserializable_string) . parse () . unwrap () ; (impl_generics , ty_generics) }
};
}
