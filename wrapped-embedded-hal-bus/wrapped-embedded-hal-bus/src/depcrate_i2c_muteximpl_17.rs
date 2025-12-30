// Generated macro for impl_17 (impl)
macro_rules! Depcrate_i2c_muteximpl_17 {
() => {
// Module: crate::i2c::mutex
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > I2c for MutexDevice < '_ , T > where T : I2c , { # [inline] fn read (& mut self , address : u8 , read : & mut [u8]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . lock () . unwrap () ; bus . read (address , read) } # [inline] fn write (& mut self , address : u8 , write : & [u8]) -> Result < () , Self :: Error > { let bus = & mut * self . bus . lock () . unwrap () ; bus . write (address , write) } # [inline] fn write_read (& mut self , address : u8 , write : & [u8] , read : & mut [u8] ,) -> Result < () , Self :: Error > { let bus = & mut * self . bus . lock () . unwrap () ; bus . write_read (address , write , read) } # [inline] fn transaction (& mut self , address : u8 , operations : & mut [embedded_hal :: i2c :: Operation < '_ >] ,) -> Result < () , Self :: Error > { let bus = & mut * self . bus . lock () . unwrap () ; bus . transaction (address , operations) } }
};
}
