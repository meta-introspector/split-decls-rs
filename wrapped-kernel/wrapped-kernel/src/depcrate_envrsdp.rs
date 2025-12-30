// Generated macro for rsdp (function)
macro_rules! Depcrate_envrsdp {
() => {
// Module: crate::env
// Provides: {"rsdp"}
// Dependencies: {}
# [doc = " Returns the RSDP physical address if available."] # [cfg (all (target_arch = "x86_64" , feature = "acpi"))] pub fn rsdp () -> Option < core :: num :: NonZero < usize > > { let rsdp = fdt () ? . find_node ("/hermit,rsdp") ? . reg () ? . next () ? . starting_address . addr () ; core :: num :: NonZero :: new (rsdp) }
};
}
