// Generated macro for DescriptorTablePointer (struct)
macro_rules! Depcrate_dtablesDescriptorTablePointer {
() => {
// Module: crate::dtables
// Provides: {"DescriptorTablePointer"}
// Dependencies: {}
# [doc = " A struct describing a pointer to a descriptor table (GDT / IDT)."] # [doc = " This is in a format suitable for giving to 'lgdt' or 'lidt'."] # [repr (C , packed)] pub struct DescriptorTablePointer < Entry > { # [doc = " Size of the DT."] pub limit : u16 , # [doc = " Pointer to the memory region containing the DT."] pub base : * const Entry , }
};
}
