// Generated macro for impl_31 (impl)
macro_rules! Depcrate_implsimpl_31 {
() => {
// Module: crate::impls
// Provides: {"impl_31"}
// Dependencies: {}
impl std :: fmt :: Display for Url { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut storage ; let to_print = if self . password . is_some () { storage = self . clone () ; storage . password = Some ("redacted" . into ()) ; & storage } else { self } ; to_print . to_bstring () . fmt (f) } }
};
}
