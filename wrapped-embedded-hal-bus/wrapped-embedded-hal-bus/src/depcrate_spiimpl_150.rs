// Generated macro for impl_150 (impl)
macro_rules! Depcrate_spiimpl_150 {
() => {
// Module: crate::spi
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (feature = "async")] # [cfg_attr (docsrs , doc (cfg (feature = "async")))] impl embedded_hal_async :: delay :: DelayNs for NoDelay { # [inline] async fn delay_ns (& mut self , _ns : u32) { no_delay_panic () ; } }
};
}
