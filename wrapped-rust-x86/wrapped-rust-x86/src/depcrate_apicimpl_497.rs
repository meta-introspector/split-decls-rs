// Generated macro for impl_497 (impl)
macro_rules! Depcrate_apicimpl_497 {
() => {
// Module: crate::apic
// Provides: {"impl_497"}
// Dependencies: {}
impl ApicId { # [doc = " Returns the Logical x2APIC ID."] # [doc = ""] # [doc = " In x2APIC mode, the 32-bit logical x2APIC ID, which can be read from LDR,"] # [doc = " is derived from the 32-bit local x2APIC ID:"] # [doc = " Logical x2APIC ID = [(x2APIC ID[19:4] « 16) | (1 « x2APIC ID[3:0])]"] pub fn x2apic_logical_id (& self) -> u32 { self . x2apic_logical_cluster_id () << 16 | 1 << self . x2apic_logical_cluster_address () } # [doc = " Returns the logical address relative to a cluster"] # [doc = " for a given APIC ID (assuming x2APIC addressing)."] pub fn x2apic_logical_cluster_address (& self) -> u32 { let d = match * self { ApicId :: XApic (id) => id as u32 , ApicId :: X2Apic (id) => id as u32 , } ; d . get_bits (0 ..= 3) } # [doc = " Returns the cluster ID a given APIC ID belongs to"] # [doc = " (assuming x2APIC addressing)."] pub fn x2apic_logical_cluster_id (& self) -> u32 { let d = match * self { ApicId :: XApic (id) => id as u32 , ApicId :: X2Apic (id) => id as u32 , } ; d . get_bits (4 ..= 19) } }
};
}
