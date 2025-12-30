// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_types_maybe_undefinedimpl_1130 {
() => {
// Module: crate::types::maybe_undefined
// Provides: {"impl_1130"}
// Dependencies: {}
impl < T > From < MaybeUndefined < T > > for Option < Option < T > > { fn from (maybe_undefined : MaybeUndefined < T >) -> Self { match maybe_undefined { MaybeUndefined :: Undefined => None , MaybeUndefined :: Null => Some (None) , MaybeUndefined :: Value (value) => Some (Some (value)) , } } }
};
}
