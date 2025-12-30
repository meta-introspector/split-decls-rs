// Generated macro for impl_76 (impl)
macro_rules! Depcrate_spiimpl_76 {
() => {
// Module: crate::spi
// Provides: {"impl_76"}
// Dependencies: {}
impl core :: fmt :: Display for ErrorKind { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Overrun => write ! (f , "The peripheral receive buffer was overrun") , Self :: ModeFault => write ! (f , "Multiple devices on the SPI bus are trying to drive the slave select pin") , Self :: FrameFormat => write ! (f , "Received data does not conform to the peripheral configuration") , Self :: ChipSelectFault => write ! (f , "An error occurred while asserting or deasserting the Chip Select pin") , Self :: Other => write ! (f , "A different error occurred. The original error may contain more information") , } } }
};
}
