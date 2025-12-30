// Generated macro for lifetimes_to_generic_params (function)
macro_rules! Depcratelifetimes_to_generic_params {
() => {
// Module: crate
// Provides: {"lifetimes_to_generic_params"}
// Dependencies: {}
fn lifetimes_to_generic_params (lv : & Punctuated < LifetimeParam , Token ! [,] >) -> Punctuated < GenericParam , Token ! [,] > { lv . iter () . map (| lt | GenericParam :: Lifetime (lt . clone ())) . collect () }
};
}
