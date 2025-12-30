// Generated macro for impl_421 (impl)
macro_rules! Depcrate_apic_ioapicimpl_421 {
() => {
// Module: crate::apic::ioapic
// Provides: {"impl_421"}
// Dependencies: {}
impl IoApic { # [doc = " Instantiate a new IoApic."] # [doc = ""] # [doc = " # Safety"] # [doc = " `addr` must point to the base of the IoApic."] pub unsafe fn new (addr : usize) -> Self { IoApic { reg : addr as * mut u32 , data : (addr + 0x10) as * mut u32 , } } pub fn disable_all (& mut self) { for i in 0 .. self . supported_interrupts () { self . write_irq (i , RedirectionEntry :: DISABLED , 0) ; } } unsafe fn read (& mut self , reg : u8) -> u32 { self . reg . write_volatile (reg as u32) ; self . data . read_volatile () } unsafe fn write (& mut self , reg : u8 , data : u32) { self . reg . write_volatile (reg as u32) ; self . data . write_volatile (data) ; } fn write_irq (& mut self , irq : u8 , flags : RedirectionEntry , dest : u8) { unsafe { self . write (REG_TABLE + 2 * irq , (T_IRQ0 + irq) as u32 | flags . bits ()) ; self . write (REG_TABLE + 2 * irq + 1 , (dest as u32) << 24) ; } } pub fn enable (& mut self , irq : u8 , cpunum : u8) { self . write_irq (irq , RedirectionEntry :: NONE , cpunum) ; } pub fn id (& mut self) -> u8 { unsafe { self . read (REG_ID) . get_bits (24 .. 28) as u8 } } pub fn version (& mut self) -> u8 { unsafe { self . read (REG_VER) . get_bits (0 .. 8) as u8 } } # [doc = " Number of supported interrupts by this IO APIC."] # [doc = ""] # [doc = " Max Redirection Entry = \"how many IRQs can this I/O APIC handle - 1\""] # [doc = " The -1 is silly so we add one back to it."] pub fn supported_interrupts (& mut self) -> u8 { unsafe { (self . read (REG_VER) . get_bits (16 .. 24) + 1) as u8 } } }
};
}
