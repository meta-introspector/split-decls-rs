// Generated macro for impl_48 (impl)
macro_rules! Depcrate_i2cimpl_48 {
() => {
// Module: crate::i2c
// Provides: {"impl_48"}
// Dependencies: {}
impl < A : AddressMode , T : I2c < A > + ? Sized > I2c < A > for & mut T { # [inline] fn read (& mut self , address : A , read : & mut [u8]) -> Result < () , Self :: Error > { T :: read (self , address , read) } # [inline] fn write (& mut self , address : A , write : & [u8]) -> Result < () , Self :: Error > { T :: write (self , address , write) } # [inline] fn write_read (& mut self , address : A , write : & [u8] , read : & mut [u8]) -> Result < () , Self :: Error > { T :: write_read (self , address , write , read) } # [inline] fn transaction (& mut self , address : A , operations : & mut [Operation < '_ >] ,) -> Result < () , Self :: Error > { T :: transaction (self , address , operations) } }
};
}
