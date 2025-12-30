// Generated macro for impl_35 (impl)
macro_rules! Depcrate_adapter_winconimpl_35 {
() => {
// Module: crate::adapter::wincon
// Provides: {"impl_35"}
// Dependencies: {}
impl WinconBytes { # [doc = " Initial state"] pub fn new () -> Self { Default :: default () } # [doc = " Strip the next segment of data"] pub fn extract_next < 's > (& 's mut self , bytes : & 's [u8]) -> WinconBytesIter < 's > { self . capture . reset () ; self . capture . printable . reserve (bytes . len ()) ; WinconBytesIter { bytes , parser : & mut self . parser , capture : & mut self . capture , } } }
};
}
