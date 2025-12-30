// Generated macro for impl_23 (impl)
macro_rules! Depcrate_spiimpl_23 {
() => {
// Module: crate::spi
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : SpiBus < Word > + ? Sized , Word : 'static + Copy > SpiBus < Word > for & mut T { # [inline] async fn read (& mut self , words : & mut [Word]) -> Result < () , Self :: Error > { T :: read (self , words) . await } # [inline] async fn write (& mut self , words : & [Word]) -> Result < () , Self :: Error > { T :: write (self , words) . await } # [inline] async fn transfer (& mut self , read : & mut [Word] , write : & [Word]) -> Result < () , Self :: Error > { T :: transfer (self , read , write) . await } # [inline] async fn transfer_in_place (& mut self , words : & mut [Word]) -> Result < () , Self :: Error > { T :: transfer_in_place (self , words) . await } # [inline] async fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) . await } }
};
}
