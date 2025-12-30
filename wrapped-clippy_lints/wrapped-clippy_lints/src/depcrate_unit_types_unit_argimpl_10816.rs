// Generated macro for impl_10816 (impl)
macro_rules! Depcrate_unit_types_unit_argimpl_10816 {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"impl_10816"}
// Dependencies: {}
impl From < MaybeTypeUncertain < '_ > > for String { fn from (value : MaybeTypeUncertain < '_ >) -> Self { match value { MaybeTypeUncertain :: Certain (sugg) => sugg . to_string () , MaybeTypeUncertain :: Uncertain (sugg) => format ! ("let _: () = {sugg}") , } } }
};
}
