// Generated macro for gen_constructor_for_field (function)
macro_rules! Depcrategen_constructor_for_field {
() => {
// Module: crate
// Provides: {"gen_constructor_for_field"}
// Dependencies: {}
fn gen_constructor_for_field (field : & Field) -> Result < TokenStream > { let ctor = match determine_field_constructor (field) ? { FieldConstructor :: Default => quote ! (:: core :: default :: Default :: default ()) , FieldConstructor :: Arbitrary => quote ! (arbitrary :: Arbitrary :: arbitrary (u) ?) , FieldConstructor :: With (function_or_closure) => quote ! ((# function_or_closure) (u) ?) , FieldConstructor :: Value (value) => quote ! (# value) , } ; Ok (ctor) }
};
}
