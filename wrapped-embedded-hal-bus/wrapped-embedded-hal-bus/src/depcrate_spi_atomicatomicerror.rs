// Generated macro for AtomicError (enum)
macro_rules! Depcrate_spi_atomicAtomicError {
() => {
// Module: crate::spi::atomic
// Provides: {"AtomicError"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " Wrapper type for errors returned by [`AtomicDevice`]."] # [cfg_attr (docsrs , doc (cfg (any (feature = "portable-atomic" , target_has_atomic = "8"))))] pub enum AtomicError < T : Error > { # [doc = " This error is returned if the SPI bus was already in use when an operation was attempted,"] # [doc = " which indicates that the driver requirements are not being met with regard to"] # [doc = " synchronization."] Busy , # [doc = " An SPI-related error occurred, and the internal error should be inspected."] Other (T) , }
};
}
