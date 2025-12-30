// Generated macro for check_t (macro)
macro_rules! Depcratecheck_t {
() => {
// Module: crate
// Provides: {"check_t"}
// Dependencies: {}
# [doc = " A helper macro to check if AsRef and AsMut are implemented for a given type."] macro_rules ! check_t { ($ t : ty) => { { fn check_ref < T : AsRef <$ t >> () { } fn propagate_ref < T1 : AsRef <$ t >, T2 : AsRef <$ t >> () { check_ref ::< Either < T1 , T2 >> () } fn check_mut < T : AsMut <$ t >> () { } fn propagate_mut < T1 : AsMut <$ t >, T2 : AsMut <$ t >> () { check_mut ::< Either < T1 , T2 >> () } } } ; }
};
}
