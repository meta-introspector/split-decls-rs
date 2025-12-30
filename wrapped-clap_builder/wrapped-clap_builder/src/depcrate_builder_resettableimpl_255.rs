// Generated macro for impl_255 (impl)
macro_rules! Depcrate_builder_resettableimpl_255 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_255"}
// Dependencies: {}
impl < T > From < Option < T > > for Resettable < T > { fn from (other : Option < T >) -> Self { match other { Some (inner) => Self :: Value (inner) , None => Self :: Reset , } } }
};
}
