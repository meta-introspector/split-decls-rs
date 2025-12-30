// Generated macro for print_symtab_symbols (function)
macro_rules! Depcrate_readobj_machoprint_symtab_symbols {
() => {
// Module: crate::readobj::macho
// Provides: {"print_symtab_symbols"}
// Dependencies: {}
fn print_symtab_symbols < Mach : MachHeader > (p : & mut Printer < '_ > , endian : Mach :: Endian , data : & [u8] , symtab : & SymtabCommand < Mach :: Endian > , state : & MachState ,) { if ! p . options . symbols { return ; } if let Some (symbols) = symtab . symbols :: < Mach , _ > (endian , data) . print_err (p) { for (index , nlist) in symbols . iter () . enumerate () { p . group ("Nlist" , | p | { p . field ("Index" , index) ; p . field_string ("String" , nlist . n_strx (endian) , nlist . name (endian , symbols . strings ()) ,) ; let n_type = nlist . n_type () ; if nlist . is_stab () { p . field_enum ("Type" , n_type , FLAGS_N_STAB) ; } else if n_type & N_TYPE == n_type { p . field_enum ("Type" , n_type , FLAGS_N_TYPE) ; } else { p . field_hex ("Type" , n_type) ; p . flags (n_type , N_TYPE , FLAGS_N_TYPE) ; p . flags (n_type , 0 , FLAGS_N_EXT) ; } let n_sect = nlist . n_sect () ; let name = state . sections . get (n_sect as usize) . map (| name | & name [..]) ; p . field_string_option ("Section" , n_sect , name) ; let n_desc = nlist . n_desc (endian) ; p . field_hex ("Desc" , n_desc) ; if nlist . is_undefined () { p . flags (n_desc , REFERENCE_TYPE , FLAGS_REFERENCE) ; } if ! nlist . is_stab () { p . flags (n_desc , 0 , FLAGS_N_DESC) ; } p . field_hex ("Value" , nlist . n_value (endian) . into ()) ; }) ; } } }
};
}
