// Generated macro for Error (trait)
macro_rules! Depcrate_spiError {
() => {
// Module: crate::spi
// Provides: {"Error"}
// Dependencies: {}
# [doc = " SPI error."] pub trait Error : Debug { # [doc = " Convert error to a generic SPI error kind."] # [doc = ""] # [doc = " By using this method, SPI errors freely defined by HAL implementations"] # [doc = " can be converted to a set of generic SPI errors upon which generic"] # [doc = " code can act."] fn kind (& self) -> ErrorKind ; }
};
}
