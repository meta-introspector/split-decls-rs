// Generated macro for print_fat_header (function)
macro_rules! Depcrate_readobj_machoprint_fat_header {
() => {
// Module: crate::readobj::macho
// Provides: {"print_fat_header"}
// Dependencies: {}
pub (super) fn print_fat_header (p : & mut Printer < '_ > , header : & macho :: FatHeader) { if ! p . options . file { return ; } p . group ("FatHeader" , | p | { p . field_hex ("Magic" , header . magic . get (BigEndian)) ; p . field ("NumberOfFatArch" , header . nfat_arch . get (BigEndian)) ; }) ; }
};
}
