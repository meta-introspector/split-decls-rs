// Generated macro for RelocationMap (struct)
macro_rules! Depcrate_readRelocationMap {
() => {
// Module: crate::read
// Provides: {"RelocationMap"}
// Dependencies: {}
# [doc = " A map from section offsets to relocation information."] # [doc = ""] # [doc = " This can be used to apply relocations to a value at a given section offset."] # [doc = " This is intended for use with DWARF in relocatable object files, and only"] # [doc = " supports relocations that are used in DWARF."] # [doc = ""] # [doc = " Returned by [`ObjectSection::relocation_map`]."] # [derive (Debug , Default)] pub struct RelocationMap (Map < u64 , RelocationMapEntry >) ;
};
}
