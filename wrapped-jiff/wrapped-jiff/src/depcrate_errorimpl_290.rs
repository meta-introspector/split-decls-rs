// Generated macro for impl_290 (impl)
macro_rules! Depcrate_errorimpl_290 {
() => {
// Module: crate::error
// Provides: {"impl_290"}
// Dependencies: {}
impl RangeError { fn new (what : & 'static str , _given : impl Into < i128 > , _min : impl Into < i128 > , _max : impl Into < i128 > ,) -> RangeError { RangeError { what , # [cfg (feature = "alloc")] given : _given . into () , # [cfg (feature = "alloc")] min : _min . into () , # [cfg (feature = "alloc")] max : _max . into () , } } }
};
}
