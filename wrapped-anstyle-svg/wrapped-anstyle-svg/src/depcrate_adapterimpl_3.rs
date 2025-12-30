// Generated macro for impl_3 (impl)
macro_rules! Depcrate_adapterimpl_3 {
() => {
// Module: crate::adapter
// Provides: {"impl_3"}
// Dependencies: {}
impl AnsiBytes { # [doc = " Initial state"] pub (crate) fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub (crate) fn extract_next < 's > (& 's mut self , bytes : & 's [u8]) -> AnsiBytesIter < 's > { self . capture . reset () ; self . capture . printable . reserve (bytes . len ()) ; AnsiBytesIter { bytes , parser : & mut self . parser , capture : & mut self . capture , } } }
};
}
