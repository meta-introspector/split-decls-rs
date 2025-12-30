// Generated macro for impl_983 (impl)
macro_rules! Depcrate_util_colorimpl_983 {
() => {
// Module: crate::util::color
// Provides: {"impl_983"}
// Dependencies: {}
impl std :: fmt :: Display for ColorChoice { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_possible_value () . expect ("no values are skipped") . get_name () . fmt (f) } }
};
}
