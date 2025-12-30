// Generated macro for print_fat_arch (function)
macro_rules! Depcrate_readobj_machoprint_fat_arch {
() => {
// Module: crate::readobj::macho
// Provides: {"print_fat_arch"}
// Dependencies: {}
pub (super) fn print_fat_arch < Arch : FatArch > (p : & mut Printer < '_ > , arch : & Arch) { if ! p . options . file { return ; } p . group ("FatArch" , | p | { print_cputype (p , arch . cputype () , arch . cpusubtype ()) ; p . field_hex ("Offset" , arch . offset () . into ()) ; p . field_hex ("Size" , arch . size () . into ()) ; p . field ("Align" , arch . align ()) ; }) ; }
};
}
