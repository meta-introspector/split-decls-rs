// Generated macro for macro_419 (macro)
macro_rules! Depcrate_apic_ioapicmacro_419 {
() => {
// Module: crate::apic::ioapic
// Provides: {"macro_419"}
// Dependencies: {}
bitflags ! { # [doc = " The redirection table starts at REG_TABLE and uses"] # [doc = " two registers to configure each interrupt."] # [doc = " The first (low) register in a pair contains configuration bits."] # [doc = " The second (high) register contains a bitmask telling which"] # [doc = " CPUs can serve that interrupt."] struct RedirectionEntry : u32 { # [doc = " Interrupt disabled"] const DISABLED = 0x00010000 ; # [doc = " Level-triggered (vs edge)"] const LEVEL = 0x00008000 ; # [doc = " Active low (vs high)"] const ACTIVELOW = 0x00002000 ; # [doc = " Destination is CPU id (vs APIC ID)"] const LOGICAL = 0x00000800 ; # [doc = " None"] const NONE = 0x00000000 ; } }
};
}
