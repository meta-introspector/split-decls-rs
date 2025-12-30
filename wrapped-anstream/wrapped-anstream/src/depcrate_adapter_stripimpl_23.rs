// Generated macro for impl_23 (impl)
macro_rules! Depcrate_adapter_stripimpl_23 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_23"}
// Dependencies: {}
impl StripBytes { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn strip_next < 's > (& 's mut self , bytes : & 's [u8]) -> StripBytesIter < 's > { StripBytesIter { bytes , state : & mut self . state , utf8parser : & mut self . utf8parser , } } }
};
}
