// Generated macro for print_load_commands (function)
macro_rules! Depcrate_readobj_machoprint_load_commands {
() => {
// Module: crate::readobj::macho
// Provides: {"print_load_commands"}
// Dependencies: {}
fn print_load_commands < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , data : & [u8] , offset : u64 , header : & Mach , state : & mut MachState ,) { if let Some (mut commands) = header . load_commands (endian , data , offset) . print_err (p) { while let Some (Some (command)) = commands . next () . print_err (p) { print_load_command (p , endian , data , header , command , state) ; } } }
};
}
