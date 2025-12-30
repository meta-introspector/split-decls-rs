// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_spiErrorKind {
() => {
// Module: crate::spi
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " SPI error kind."] # [doc = ""] # [doc = " This represents a common set of SPI operation errors. HAL implementations are"] # [doc = " free to define more specific or additional error types. However, by providing"] # [doc = " a mapping to these common SPI errors, generic code can still react to them."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] # [non_exhaustive] pub enum ErrorKind { # [doc = " The peripheral receive buffer was overrun."] Overrun , # [doc = " Multiple devices on the SPI bus are trying to drive the slave select pin, e.g. in a multi-master setup."] ModeFault , # [doc = " Received data does not conform to the peripheral configuration."] FrameFormat , # [doc = " An error occurred while asserting or deasserting the Chip Select pin."] ChipSelectFault , # [doc = " A different error occurred. The original error may contain more information."] Other , }
};
}
