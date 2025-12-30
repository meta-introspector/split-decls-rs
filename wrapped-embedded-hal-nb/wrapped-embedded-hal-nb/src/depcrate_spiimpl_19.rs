// Generated macro for impl_19 (impl)
macro_rules! Depcrate_spiimpl_19 {
() => {
// Module: crate::spi
// Provides: {"impl_19"}
// Dependencies: {}
impl < T : FullDuplex < Word > + ? Sized , Word : Copy > FullDuplex < Word > for & mut T { # [inline] fn read (& mut self) -> nb :: Result < Word , Self :: Error > { T :: read (self) } # [inline] fn write (& mut self , word : Word) -> nb :: Result < () , Self :: Error > { T :: write (self , word) } }
};
}
