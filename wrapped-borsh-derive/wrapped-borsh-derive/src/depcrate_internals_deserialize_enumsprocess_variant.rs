// Generated macro for process_variant (function)
macro_rules! Depcrate_internals_deserialize_enumsprocess_variant {
() => {
// Module: crate::internals::deserialize::enums
// Provides: {"process_variant"}
// Dependencies: {}
fn process_variant (variant : & Variant , cratename : & Path , generics : & mut deserialize :: GenericsOutput ,) -> syn :: Result < TokenStream2 > { let mut body = TokenStream2 :: new () ; match & variant . fields { Fields :: Named (fields) => { for field in & fields . named { deserialize :: process_field (field , cratename , & mut body , generics) ? ; } body = quote ! { { # body } } ; } Fields :: Unnamed (fields) => { for field in fields . unnamed . iter () { deserialize :: process_field (field , cratename , & mut body , generics) ? ; } body = quote ! { (# body) } ; } Fields :: Unit => { } } Ok (body) }
};
}
