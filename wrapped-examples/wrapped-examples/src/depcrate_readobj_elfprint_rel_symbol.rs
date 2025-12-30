// Generated macro for print_rel_symbol (function)
macro_rules! Depcrate_readobj_elfprint_rel_symbol {
() => {
// Module: crate::readobj::elf
// Provides: {"print_rel_symbol"}
// Dependencies: {}
fn print_rel_symbol < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , symbols : Option < SymbolTable < '_ , Elf > > , index : Option < SymbolIndex > ,) { let Some (index) = index else { p . field_hex ("Symbol" , 0) ; return ; } ; let name = symbols . and_then (| symbols | { symbols . symbol (index) . and_then (| symbol | symbol . name (endian , symbols . strings ())) . print_err (p) }) ; p . field_string_option ("Symbol" , index . 0 , name) ; }
};
}
