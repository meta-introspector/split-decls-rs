// Generated macro for FatHeader (struct)
macro_rules! Depcrate_machoFatHeader {
() => {
// Module: crate::macho
// Provides: {"FatHeader"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FatHeader { # [doc = " FAT_MAGIC or FAT_MAGIC_64"] pub magic : U32 < BigEndian > , # [doc = " number of structs that follow"] pub nfat_arch : U32 < BigEndian > , }
};
}
