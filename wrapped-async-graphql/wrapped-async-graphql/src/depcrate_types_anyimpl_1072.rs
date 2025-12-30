// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_types_anyimpl_1072 {
() => {
// Module: crate::types::any
// Provides: {"impl_1072"}
// Dependencies: {}
impl < T : Into < Value > > From < T > for Any { fn from (value : T) -> Any { Any (value . into ()) } }
};
}
