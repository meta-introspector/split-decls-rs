// Generated macro for print_mach_header (function)
macro_rules! Depcrate_readobj_machoprint_mach_header {
() => {
// Module: crate::readobj::macho
// Provides: {"print_mach_header"}
// Dependencies: {}
fn print_mach_header < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , header : & Mach) { if ! p . options . file { return ; } p . group ("MachHeader" , | p | { p . field_hex ("Magic" , header . magic ()) ; print_cputype (p , header . cputype (endian) , header . cpusubtype (endian)) ; p . field_enum ("FileType" , header . filetype (endian) , FLAGS_MH_FILETYPE) ; p . field ("NumberOfCmds" , header . ncmds (endian)) ; p . field_hex ("SizeOfCmds" , header . sizeofcmds (endian)) ; p . field_enum ("Flags" , header . flags (endian) , FLAGS_MH) ; }) ; }
};
}
