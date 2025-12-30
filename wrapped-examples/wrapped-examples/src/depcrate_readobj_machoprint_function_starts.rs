// Generated macro for print_function_starts (function)
macro_rules! Depcrate_readobj_machoprint_function_starts {
() => {
// Module: crate::readobj::macho
// Provides: {"print_function_starts"}
// Dependencies: {}
fn print_function_starts < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , linkedit : & LinkeditDataCommand < Mach :: Endian > , state : & MachState ,) { let Some (function_starts) = linkedit . function_starts (endian , state . linkedit_data , state . text_segment_addr) . print_err (p) else { return ; } ; p . group ("FunctionStarts" , | p | { for addr in function_starts { addr . print_err (p) . map (| addr | p . field_hex ("Address" , addr)) ; } }) ; }
};
}
