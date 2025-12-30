// Generated macro for impl_21 (impl)
macro_rules! Depcrate_spiimpl_21 {
() => {
// Module: crate::spi
// Provides: {"impl_21"}
// Dependencies: {}
impl < Word : Copy + 'static , T : SpiDevice < Word > + ? Sized > SpiDevice < Word > for & mut T { # [inline] async fn transaction (& mut self , operations : & mut [Operation < '_ , Word >] ,) -> Result < () , Self :: Error > { T :: transaction (self , operations) . await } # [inline] async fn read (& mut self , buf : & mut [Word]) -> Result < () , Self :: Error > { T :: read (self , buf) . await } # [inline] async fn write (& mut self , buf : & [Word]) -> Result < () , Self :: Error > { T :: write (self , buf) . await } # [inline] async fn transfer (& mut self , read : & mut [Word] , write : & [Word]) -> Result < () , Self :: Error > { T :: transfer (self , read , write) . await } # [inline] async fn transfer_in_place (& mut self , buf : & mut [Word]) -> Result < () , Self :: Error > { T :: transfer_in_place (self , buf) . await } }
};
}
