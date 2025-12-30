// Generated macro for validate_attrs (function)
macro_rules! Depcrate_systemvalidate_attrs {
() => {
// Module: crate::system
// Provides: {"validate_attrs"}
// Dependencies: {}
fn validate_attrs (attrs : & [Attribute]) -> Result < () > { let mut no_mangle_found = false ; for attr in attrs { if attr . path () . is_ident ("unsafe") && let Ok (ident) = attr . parse_args :: < Ident > () && ident == "no_mangle" { no_mangle_found = true ; continue ; } if ! attr . path () . is_ident ("cfg") && ! attr . path () . is_ident ("doc") { bail ! (attr , "#[system] functions may only have `#[doc]`, `#[unsafe(no_mangle)]` and `#[cfg]` attributes") ; } } if ! no_mangle_found { bail ! (attrs . first () , "#[system] functions must have `#[unsafe(no_mangle)]` attribute") ; } Ok (()) }
};
}
