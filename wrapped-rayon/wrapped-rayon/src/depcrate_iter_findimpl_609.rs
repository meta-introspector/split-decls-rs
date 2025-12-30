// Generated macro for impl_609 (impl)
macro_rules! Depcrate_iter_findimpl_609 {
() => {
// Module: crate::iter::find
// Provides: {"impl_609"}
// Dependencies: {}
impl < T > Reducer < Option < T > > for FindReducer { fn reduce (self , left : Option < T > , right : Option < T >) -> Option < T > { left . or (right) } }
};
}
