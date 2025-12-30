// Generated macro for impl_146 (impl)
macro_rules! Depcrate_spiimpl_146 {
() => {
// Module: crate::spi
// Provides: {"impl_146"}
// Dependencies: {}
impl < BUS , CS > Error for DeviceError < BUS , CS > where BUS : Error + Debug , CS : Debug , { # [inline] fn kind (& self) -> ErrorKind { match self { Self :: Spi (e) => e . kind () , Self :: Cs (_) => ErrorKind :: ChipSelectFault , } } }
};
}
