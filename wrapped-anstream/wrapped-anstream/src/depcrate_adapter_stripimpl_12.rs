// Generated macro for impl_12 (impl)
macro_rules! Depcrate_adapter_stripimpl_12 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_12"}
// Dependencies: {}
impl StripStr { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn strip_next < 's > (& 's mut self , data : & 's str) -> StripStrIter < 's > { StripStrIter { bytes : data . as_bytes () , state : & mut self . state , } } }
};
}
