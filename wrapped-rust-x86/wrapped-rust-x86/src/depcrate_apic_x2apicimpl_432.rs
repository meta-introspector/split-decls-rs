// Generated macro for impl_432 (impl)
macro_rules! Depcrate_apic_x2apicimpl_432 {
() => {
// Module: crate::apic::x2apic
// Provides: {"impl_432"}
// Dependencies: {}
impl X2APIC { # [doc = " Create a new x2APIC driver object for the local core."] # [doc = ""] # [doc = " # Notes"] # [doc = " The object needs to be initialized by calling `attach()` first which"] # [doc = " enables the x2APIC. There should be only one x2APIC object created per"] # [doc = " core."] pub const fn new () -> Self { X2APIC { base : 0x0 } } # [doc = " Attach to APIC (enable x2APIC mode, initialize LINT0)"] pub fn attach (& mut self) { unsafe { self . base = rdmsr (IA32_APIC_BASE) ; self . base . set_bit (10 , true) ; self . base . set_bit (11 , true) ; wrmsr (IA32_APIC_BASE , self . base) ; let svr : u64 = 1 << 8 | 15 ; wrmsr (IA32_X2APIC_SIVR , svr) ; let lint0 = 1 << 16 | (1 << 15) | (0b111 << 8) | 0x20 ; wrmsr (IA32_X2APIC_LVT_LINT0 , lint0) ; let _esr = rdmsr (IA32_X2APIC_ESR) ; } } # [doc = " Detach from APIC (disable x2APIC and xAPIC mode)."] pub fn detach (& mut self) { unsafe { self . base = rdmsr (IA32_APIC_BASE) ; self . base . set_bit (10 , false) ; self . base . set_bit (11 , false) ; wrmsr (IA32_APIC_BASE , self . base) ; } } # [doc = " Send an IPI to yourself."] # [doc = ""] # [doc = " # Safety"] # [doc = " Will interrupt core with `vector`."] pub unsafe fn send_self_ipi (& self , vector : u64) { wrmsr (IA32_X2APIC_SELF_IPI , vector) ; } }
};
}
