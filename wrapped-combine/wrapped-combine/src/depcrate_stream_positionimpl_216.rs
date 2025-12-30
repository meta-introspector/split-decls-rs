// Generated macro for impl_216 (impl)
macro_rules! Depcrate_stream_positionimpl_216 {
() => {
// Module: crate::stream::position
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a > RangePositioner < char , & 'a str > for SourcePosition { fn update_range (& mut self , range : & & 'a str) { for c in range . chars () { self . update (& c) ; } } }
};
}
