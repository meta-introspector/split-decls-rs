// Generated macro for impl_144 (impl)
macro_rules! Depcrate_spiimpl_144 {
() => {
// Module: crate::spi
// Provides: {"impl_144"}
// Dependencies: {}
impl < BUS : Display , CS : Display > Display for DeviceError < BUS , CS > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { Self :: Spi (bus) => write ! (f , "SPI bus error: {bus}") , Self :: Cs (cs) => write ! (f , "SPI CS error: {cs}") , } } }
};
}
