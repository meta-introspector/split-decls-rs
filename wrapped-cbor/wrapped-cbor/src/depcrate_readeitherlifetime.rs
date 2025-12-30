// Generated macro for EitherLifetime (enum)
macro_rules! Depcrate_readEitherLifetime {
() => {
// Module: crate::read
// Provides: {"EitherLifetime"}
// Dependencies: {}
# [doc = " Represents a buffer with one of two lifetimes."] pub enum EitherLifetime < 'short , 'long > { # [doc = " The short lifetime"] Short (& 'short [u8]) , # [doc = " The long lifetime"] Long (& 'long [u8]) , }
};
}
