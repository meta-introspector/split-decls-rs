// Generated macro for print_ident (function)
macro_rules! Depcrate_readobj_elfprint_ident {
() => {
// Module: crate::readobj::elf
// Provides: {"print_ident"}
// Dependencies: {}
fn print_ident (p : & mut Printer < '_ > , ident : & Ident) { p . field ("Magic" , format ! ("{:X?}" , ident . magic)) ; p . field_enum ("Class" , ident . class , FLAGS_EI_CLASS) ; p . field_enum ("Data" , ident . data , FLAGS_EI_DATA) ; p . field_enum ("Version" , ident . version , FLAGS_EV) ; p . field_enum ("OsAbi" , ident . os_abi , FLAGS_EI_OSABI) ; p . field_hex ("AbiVersion" , ident . abi_version) ; p . field ("Unused" , format ! ("{:X?}" , ident . padding)) ; }
};
}
