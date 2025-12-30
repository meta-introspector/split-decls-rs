// Generated macro for impl_81 (impl)
macro_rules! Depcrate_spiimpl_81 {
() => {
// Module: crate::spi
// Provides: {"impl_81"}
// Dependencies: {}
impl < Word : Copy + 'static , T : SpiDevice < Word > + ? Sized > SpiDevice < Word > for & mut T { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { T :: transaction (self , operations) } # [inline] fn read (& mut self , buf : & mut [Word]) -> Result < () , Self :: Error > { T :: read (self , buf) } # [inline] fn write (& mut self , buf : & [Word]) -> Result < () , Self :: Error > { T :: write (self , buf) } # [inline] fn transfer (& mut self , read : & mut [Word] , write : & [Word]) -> Result < () , Self :: Error > { T :: transfer (self , read , write) } # [inline] fn transfer_in_place (& mut self , buf : & mut [Word]) -> Result < () , Self :: Error > { T :: transfer_in_place (self , buf) } }
};
}
