// Generated macro for impl_62 (impl)
macro_rules! Depcrate_paramsimpl_62 {
() => {
// Module: crate::params
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > Iter < 'a > { # [doc = " Create a new [`Iter`]."] fn new (s : & 'a str) -> Self { if s . is_empty () { Self { inner : None } } else { Self { inner : Some (s . split (PARAMS_DELIMITER)) , } } } }
};
}
