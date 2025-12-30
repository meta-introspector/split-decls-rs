// Generated macro for impl_83 (impl)
macro_rules! Depcrate_spiimpl_83 {
() => {
// Module: crate::spi
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : SpiBus < Word > + ? Sized , Word : Copy + 'static > SpiBus < Word > for & mut T { # [inline] fn read (& mut self , words : & mut [Word]) -> Result < () , Self :: Error > { T :: read (self , words) } # [inline] fn write (& mut self , words : & [Word]) -> Result < () , Self :: Error > { T :: write (self , words) } # [inline] fn transfer (& mut self , read : & mut [Word] , write : & [Word]) -> Result < () , Self :: Error > { T :: transfer (self , read , write) } # [inline] fn transfer_in_place (& mut self , words : & mut [Word]) -> Result < () , Self :: Error > { T :: transfer_in_place (self , words) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) } }
};
}
