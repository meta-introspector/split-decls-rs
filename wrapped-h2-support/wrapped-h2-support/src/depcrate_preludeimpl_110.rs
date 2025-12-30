// Generated macro for impl_110 (impl)
macro_rules! Depcrate_preludeimpl_110 {
() => {
// Module: crate::prelude
// Provides: {"impl_110"}
// Dependencies: {}
impl MockH2 for tokio_test :: io :: Builder { fn handshake (& mut self) -> & mut Self { self . handshake_read_settings (frames :: SETTINGS) } fn handshake_read_settings (& mut self , settings : & [u8]) -> & mut Self { self . write (MAGIC_PREFACE) . write (frames :: SETTINGS) . read (settings) . read (frames :: SETTINGS_ACK) } }
};
}
