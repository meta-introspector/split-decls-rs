// Generated macro for impl_486 (impl)
macro_rules! Depcrate_apic_xapicimpl_486 {
() => {
// Module: crate::apic::xapic
// Provides: {"impl_486"}
// Dependencies: {}
impl XAPIC < '_ > { # [doc = " Create a new xAPIC object for the local CPU."] # [doc = ""] # [doc = " Pass the xAPCI region which is at XXX unless you have"] # [doc = " relocated the region."] pub fn new < 'a > (apic_region : & 'a mut [u32]) -> XAPIC { unsafe { XAPIC { mmio_region : apic_region , base : rdmsr (IA32_APIC_BASE) , } } } # [doc = " Attach driver to the xAPIC (enables device)."] pub fn attach (& mut self) { unsafe { self . base = rdmsr (IA32_APIC_BASE) ; self . base . set_bit (11 , true) ; wrmsr (IA32_APIC_BASE , self . base) ; let svr : u32 = 1 << 8 | 15 ; self . write (ApicRegister :: XAPIC_SVR , svr) ; } } # [doc = " Detach driver form the xAPIC (disables device)."] pub fn detach (& mut self) { unsafe { self . base = rdmsr (IA32_APIC_BASE) ; self . base . set_bit (11 , false) ; wrmsr (IA32_APIC_BASE , self . base) ; } } # [doc = " Read a register from the MMIO region."] fn read (& self , offset : ApicRegister) -> u32 { assert ! (offset as usize % 4 == 0) ; let index = offset as usize / 4 ; unsafe { core :: ptr :: read_volatile (& self . mmio_region [index]) } } # [doc = " write a register in the MMIO region."] fn write (& mut self , offset : ApicRegister , val : u32) { assert ! (offset as usize % 4 == 0) ; let index = offset as usize / 4 ; unsafe { core :: ptr :: write_volatile (& mut self . mmio_region [index] , val) } } }
};
}
