// Generated macro for impl_9 (impl)
macro_rules! Depcrate_i2c_refcellimpl_9 {
() => {
// Module: crate::i2c::refcell
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > I2c for RefCellDevice < '_ , T > where T : I2c , { # [inline] fn read (& mut self , address : u8 , read : & mut [u8]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; bus . read (address , read) } # [inline] fn write (& mut self , address : u8 , write : & [u8]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; bus . write (address , write) } # [inline] fn write_read (& mut self , address : u8 , write : & [u8] , read : & mut [u8] ,) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; bus . write_read (address , write , read) } # [inline] fn transaction (& mut self , address : u8 , operations : & mut [embedded_hal :: i2c :: Operation < '_ >] ,) -> Result < () , Self :: Error > { let bus = & mut * self . bus . borrow_mut () ; bus . transaction (address , operations) } }
};
}
