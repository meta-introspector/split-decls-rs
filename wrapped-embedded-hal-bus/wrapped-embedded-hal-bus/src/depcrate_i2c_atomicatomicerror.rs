// Generated macro for AtomicError (enum)
macro_rules! Depcrate_i2c_atomicAtomicError {
() => {
// Module: crate::i2c::atomic
// Provides: {"AtomicError"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " Wrapper type for errors originating from the atomically-checked I2C bus manager."] pub enum AtomicError < T : Error > { # [doc = " This error is returned if the I2C bus was already in use when an operation was attempted,"] # [doc = " which indicates that the driver requirements are not being met with regard to"] # [doc = " synchronization."] Busy , # [doc = " An I2C-related error occurred, and the internal error should be inspected."] Other (T) , }
};
}
