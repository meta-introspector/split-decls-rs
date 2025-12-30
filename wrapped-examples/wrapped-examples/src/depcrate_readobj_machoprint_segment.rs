// Generated macro for print_segment (function)
macro_rules! Depcrate_readobj_machoprint_segment {
() => {
// Module: crate::readobj::macho
// Provides: {"print_segment"}
// Dependencies: {}
fn print_segment < S : Segment > (p : & mut Printer < '_ > , endian : S :: Endian , data : & [u8] , segment : & S , section_data : & [u8] , state : & mut MachState ,) { if ! p . options . macho_load_commands && ! p . options . segments && ! p . options . sections && ! p . options . relocations { return ; } p . group ("SegmentCommand" , | p | { p . field_enum ("Cmd" , segment . cmd (endian) , FLAGS_LC) ; p . field_hex ("CmdSize" , segment . cmdsize (endian)) ; p . field_inline_string ("SegmentName" , segment . name ()) ; if p . options . macho_load_commands || p . options . segments { p . field_hex ("VmAddress" , segment . vmaddr (endian) . into ()) ; p . field_hex ("VmSize" , segment . vmsize (endian) . into ()) ; p . field_hex ("FileOffset" , segment . fileoff (endian) . into ()) ; p . field_hex ("FileSize" , segment . filesize (endian) . into ()) ; p . field_hex ("MaxProt" , segment . maxprot (endian)) ; p . flags (segment . maxprot (endian) , 0 , FLAGS_VM) ; p . field_hex ("InitProt" , segment . initprot (endian)) ; p . flags (segment . initprot (endian) , 0 , FLAGS_VM) ; p . field ("NumberOfSections" , segment . nsects (endian)) ; p . field_hex ("Flags" , segment . flags (endian)) ; p . flags (segment . flags (endian) , 0 , FLAGS_SG) ; } if let Some (sections) = segment . sections (endian , section_data) . print_err (p) { for section in sections { state . section_index += 1 ; print_section (p , endian , data , section , state) ; } } }) ; }
};
}
