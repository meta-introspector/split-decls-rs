// Generated macro for XAPIC (struct)
macro_rules! Depcrate_apic_xapicXAPIC {
() => {
// Module: crate::apic::xapic
// Provides: {"XAPIC"}
// Dependencies: {}
# [doc = " State for the XAPIC driver."] # [allow (clippy :: clippy :: upper_case_acronyms)] pub struct XAPIC < 'a > { # [doc = " Reference to the xAPCI region"] mmio_region : & 'a mut [u32] , # [doc = " Initial APIC Base register value."] base : u64 , }
};
}
