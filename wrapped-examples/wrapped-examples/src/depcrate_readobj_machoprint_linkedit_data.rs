// Generated macro for print_linkedit_data (function)
macro_rules! Depcrate_readobj_machoprint_linkedit_data {
() => {
// Module: crate::readobj::macho
// Provides: {"print_linkedit_data"}
// Dependencies: {}
fn print_linkedit_data < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , linkedit : & LinkeditDataCommand < Mach :: Endian > , state : & MachState ,) { let cmd = linkedit . cmd . get (endian) ; let function_starts = p . options . macho_function_starts && cmd == macho :: LC_FUNCTION_STARTS ; let exports_trie = p . options . macho_exports_trie && cmd == macho :: LC_DYLD_EXPORTS_TRIE ; if ! p . options . macho_load_commands && ! function_starts && ! exports_trie { return ; } p . group ("LinkeditDataCommand" , | p | { p . field_enum ("Cmd" , cmd , FLAGS_LC) ; p . field_hex ("CmdSize" , linkedit . cmdsize . get (endian)) ; p . field_hex ("DataOffset" , linkedit . dataoff . get (endian)) ; p . field_hex ("DataSize" , linkedit . datasize . get (endian)) ; if function_starts { print_function_starts :: < Mach > (p , endian , linkedit , state) ; } if exports_trie { print_exports_trie :: < Mach > (p , endian , linkedit , state) ; } }) ; }
};
}
