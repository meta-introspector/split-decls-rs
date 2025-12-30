// Generated macro for impl_37 (impl)
macro_rules! Depcrate_i2c_atomicimpl_37 {
() => {
// Module: crate::i2c::atomic
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > I2c for AtomicDevice < '_ , T > where T : I2c , { # [inline] fn read (& mut self , address : u8 , read : & mut [u8]) -> Result < () , Self :: Error > { self . lock (| bus | bus . read (address , read)) } # [inline] fn write (& mut self , address : u8 , write : & [u8]) -> Result < () , Self :: Error > { self . lock (| bus | bus . write (address , write)) } # [inline] fn write_read (& mut self , address : u8 , write : & [u8] , read : & mut [u8] ,) -> Result < () , Self :: Error > { self . lock (| bus | bus . write_read (address , write , read)) } # [inline] fn transaction (& mut self , address : u8 , operations : & mut [embedded_hal :: i2c :: Operation < '_ >] ,) -> Result < () , Self :: Error > { self . lock (| bus | bus . transaction (address , operations)) } }
};
}
