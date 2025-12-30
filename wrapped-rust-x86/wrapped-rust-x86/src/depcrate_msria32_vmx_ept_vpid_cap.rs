// Generated macro for IA32_VMX_EPT_VPID_CAP (const)
macro_rules! Depcrate_msrIA32_VMX_EPT_VPID_CAP {
() => {
// Module: crate::msr
// Provides: {"IA32_VMX_EPT_VPID_CAP"}
// Dependencies: {}
# [doc = " If ( CPUID.01H:ECX.\\[bit 5\\],  IA32_VMX_PROCBASED_C TLS\\[bit 63\\], and either  IA32_VMX_PROCBASED_C TLS2\\[bit 33\\] or  IA32_VMX_PROCBASED_C TLS2\\[bit 37\\])"] pub const IA32_VMX_EPT_VPID_CAP : u32 = 0x48c ;
};
}
