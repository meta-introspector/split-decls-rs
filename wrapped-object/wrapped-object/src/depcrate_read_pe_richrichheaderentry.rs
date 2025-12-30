// Generated macro for RichHeaderEntry (struct)
macro_rules! Depcrate_read_pe_richRichHeaderEntry {
() => {
// Module: crate::read::pe::rich
// Provides: {"RichHeaderEntry"}
// Dependencies: {}
# [doc = " A PE rich header entry after it has been unmasked."] # [doc = ""] # [doc = " See [`pe::MaskedRichHeaderEntry`]."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct RichHeaderEntry { # [doc = " ID of the component."] pub comp_id : u32 , # [doc = " Number of times this component has been used when building this PE."] pub count : u32 , }
};
}
