// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_types_maybe_undefinedimpl_1131 {
() => {
// Module: crate::types::maybe_undefined
// Provides: {"impl_1131"}
// Dependencies: {}
impl < T > From < Option < Option < T > > > for MaybeUndefined < T > { fn from (value : Option < Option < T > >) -> Self { match value { Some (Some (value)) => Self :: Value (value) , Some (None) => Self :: Null , None => Self :: Undefined , } } }
};
}
