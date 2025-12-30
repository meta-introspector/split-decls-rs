// Generated macro for print_symtab (function)
macro_rules! Depcrate_readobj_machoprint_symtab {
() => {
// Module: crate::readobj::macho
// Provides: {"print_symtab"}
// Dependencies: {}
fn print_symtab < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , data : & [u8] , symtab : & SymtabCommand < Mach :: Endian > , state : & MachState ,) { if ! p . options . macho_load_commands && ! p . options . symbols { return ; } p . group ("SymtabCommand" , | p | { p . field_enum ("Cmd" , symtab . cmd . get (endian) , FLAGS_LC) ; p . field_hex ("CmdSize" , symtab . cmdsize . get (endian)) ; p . field_hex ("SymbolOffset" , symtab . symoff . get (endian)) ; p . field_hex ("NumberOfSymbols" , symtab . nsyms . get (endian)) ; p . field_hex ("StringOffset" , symtab . stroff . get (endian)) ; p . field_hex ("StringSize" , symtab . strsize . get (endian)) ; print_symtab_symbols :: < Mach > (p , endian , data , symtab , state) ; }) ; }
};
}
