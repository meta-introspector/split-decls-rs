// Generated macro for impl_84 (impl)
macro_rules! Depcrate_aot_shells_shellimpl_84 {
() => {
// Module: crate::aot::shells::shell
// Provides: {"impl_84"}
// Dependencies: {}
impl Display for Shell { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_possible_value () . expect ("no values are skipped") . get_name () . fmt (f) } }
};
}
