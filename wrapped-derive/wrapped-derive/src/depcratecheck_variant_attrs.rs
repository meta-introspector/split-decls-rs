// Generated macro for check_variant_attrs (function)
macro_rules! Depcratecheck_variant_attrs {
() => {
// Module: crate
// Provides: {"check_variant_attrs"}
// Dependencies: {}
fn check_variant_attrs (variant : & Variant) -> Result < () > { for attr in & variant . attrs { if attr . path () . is_ident (ARBITRARY_ATTRIBUTE_NAME) { return Err (Error :: new_spanned (attr , format ! ("invalid `{}` attribute. it is unsupported on enum variants. try applying it to a field of the variant instead" , ARBITRARY_ATTRIBUTE_NAME) ,)) ; } } Ok (()) }
};
}
