// Generated macro for ApicControl (trait)
macro_rules! Depcrate_apicApicControl {
() => {
// Module: crate::apic
// Provides: {"ApicControl"}
// Dependencies: {}
# [doc = " Abstracts common interface of local APIC (x2APIC, xAPIC) hardware devices."] pub trait ApicControl { # [doc = " Is a bootstrap processor?"] fn bsp (& self) -> bool ; # [doc = " Return APIC ID."] fn id (& self) -> u32 ; # [doc = " Returns the logical APIC ID."] fn logical_id (& self) -> u32 ; # [doc = " Read APIC version"] fn version (& self) -> u32 ; # [doc = " End Of Interrupt -- Acknowledge interrupt delivery."] fn eoi (& mut self) ; # [doc = " Enable TSC deadline timer."] fn tsc_enable (& mut self , vector : u8) ; # [doc = " Set TSC deadline value."] fn tsc_set (& self , value : u64) ; # [doc = " Send a INIT IPI to a core."] # [doc = ""] # [doc = " # Safety"] # [doc = " Should only be used to reset or boot a new core."] unsafe fn ipi_init (& mut self , core : ApicId) ; # [doc = " Deassert INIT IPI."] # [doc = ""] # [doc = " # Safety"] # [doc = " Should only be used to reset or boot a new core."] unsafe fn ipi_init_deassert (& mut self) ; # [doc = " Send a STARTUP IPI to a core."] # [doc = ""] # [doc = " # Safety"] # [doc = " Should only be used to reset or boot a new core."] unsafe fn ipi_startup (& mut self , core : ApicId , start_page : u8) ; # [doc = " Send a generic IPI."] # [doc = ""] # [doc = " # Safety"] # [doc = " Interrupts one or multiple cores."] unsafe fn send_ipi (& mut self , icr : Icr) ; }
};
}
