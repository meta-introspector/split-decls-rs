// Generated macro for IA32_X2APIC_EOI (const)
macro_rules! Depcrate_msrIA32_X2APIC_EOI {
() => {
// Module: crate::msr
// Provides: {"IA32_X2APIC_EOI"}
// Dependencies: {}
# [doc = " x2APIC End of Interrupt. If ( CPUID.01H:ECX.\\[bit 21\\]  = 1 )"] pub const IA32_X2APIC_EOI : u32 = 0x80b ;
};
}
