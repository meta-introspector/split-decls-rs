// Generated macro for as_array (macro)
macro_rules! Depcrate_convertas_array {
() => {
// Module: crate::convert
// Provides: {"as_array"}
// Dependencies: {}
macro_rules ! as_array { ($ input : expr , $ len : expr) => { { { # [inline (always)] fn as_array < T > (slice : & [T]) -> & [T ; $ len] { core :: convert :: TryFrom :: try_from (slice) . unwrap () } as_array ($ input) } } } ; }
};
}
