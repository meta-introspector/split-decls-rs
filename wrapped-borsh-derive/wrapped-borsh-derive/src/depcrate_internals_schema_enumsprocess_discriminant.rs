// Generated macro for process_discriminant (function)
macro_rules! Depcrate_internals_schema_enumsprocess_discriminant {
() => {
// Module: crate::internals::schema::enums
// Provides: {"process_discriminant"}
// Dependencies: {}
fn process_discriminant (variant_ident : & Ident , info : DiscriminantInfo < '_ > ,) -> syn :: Result < TokenStream2 > { info . discriminants . get (variant_ident , info . use_discriminant , info . variant_idx) }
};
}
