// Generated macro for print_section (function)
macro_rules! Depcrate_readobj_machoprint_section {
() => {
// Module: crate::readobj::macho
// Provides: {"print_section"}
// Dependencies: {}
fn print_section < S : Section > (p : & mut Printer < '_ > , endian : S :: Endian , data : & [u8] , section : & S , state : & mut MachState ,) { if ! p . options . sections && ! (p . options . relocations && section . nreloc (endian) != 0) { return ; } p . group ("Section" , | p | { p . field ("Index" , state . section_index) ; p . field_inline_string ("SectionName" , section . name ()) ; p . field_inline_string ("SegmentName" , section . segment_name ()) ; if p . options . sections { p . field_hex ("Address" , section . addr (endian) . into ()) ; p . field_hex ("Size" , section . size (endian) . into ()) ; p . field_hex ("Offset" , section . offset (endian)) ; p . field_hex ("Align" , section . align (endian)) ; p . field_hex ("RelocationOffset" , section . reloff (endian)) ; p . field_hex ("NumberOfRelocations" , section . nreloc (endian)) ; let flags = section . flags (endian) ; if flags & SECTION_TYPE == flags { p . field_enum ("Flags" , flags , FLAGS_S_TYPE) ; } else { p . field_hex ("Flags" , section . flags (endian)) ; p . flags (flags , SECTION_TYPE , FLAGS_S_TYPE) ; p . flags (flags , 0 , FLAGS_S_ATTR) ; } } print_section_relocations (p , endian , data , section , state) ; }) ; }
};
}
