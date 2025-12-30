// Generated macro for ErrorType (trait)
macro_rules! Depcrate_spiErrorType {
() => {
// Module: crate::spi
// Provides: {"ErrorType"}
// Dependencies: {}
# [doc = " SPI error type trait."] # [doc = ""] # [doc = " This just defines the error type, to be used by the other SPI traits."] pub trait ErrorType { # [doc = " Error type."] type Error : Error ; }
};
}
