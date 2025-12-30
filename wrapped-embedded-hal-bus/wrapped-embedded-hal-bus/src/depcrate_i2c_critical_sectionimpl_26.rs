// Generated macro for impl_26 (impl)
macro_rules! Depcrate_i2c_critical_sectionimpl_26 {
() => {
// Module: crate::i2c::critical_section
// Provides: {"impl_26"}
// Dependencies: {}
impl < T > I2c for CriticalSectionDevice < '_ , T > where T : I2c , { # [inline] fn read (& mut self , address : u8 , read : & mut [u8]) -> Result < () , Self :: Error > { critical_section :: with (| cs | { let bus = & mut * self . bus . borrow_ref_mut (cs) ; bus . read (address , read) }) } # [inline] fn write (& mut self , address : u8 , write : & [u8]) -> Result < () , Self :: Error > { critical_section :: with (| cs | { let bus = & mut * self . bus . borrow_ref_mut (cs) ; bus . write (address , write) }) } # [inline] fn write_read (& mut self , address : u8 , write : & [u8] , read : & mut [u8] ,) -> Result < () , Self :: Error > { critical_section :: with (| cs | { let bus = & mut * self . bus . borrow_ref_mut (cs) ; bus . write_read (address , write , read) }) } # [inline] fn transaction (& mut self , address : u8 , operations : & mut [embedded_hal :: i2c :: Operation < '_ >] ,) -> Result < () , Self :: Error > { critical_section :: with (| cs | { let bus = & mut * self . bus . borrow_ref_mut (cs) ; bus . transaction (address , operations) }) } }
};
}
