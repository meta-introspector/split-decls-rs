// Generated macro for print_gnu_verneed (function)
macro_rules! Depcrate_readobj_elfprint_gnu_verneed {
() => {
// Module: crate::readobj::elf
// Provides: {"print_gnu_verneed"}
// Dependencies: {}
fn print_gnu_verneed < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . elf_versions { return ; } if let Some (Some ((mut verneeds , link))) = section . gnu_verneed (endian , data) . print_err (p) { let strings = sections . strings (endian , data , link) . unwrap_or_default () ; while let Some (Some ((verneed , mut vernauxs))) = verneeds . next () . print_err (p) { p . group ("VersionNeed" , | p | { p . field ("Version" , verneed . vn_version . get (endian)) ; p . field ("AuxCount" , verneed . vn_cnt . get (endian)) ; p . field_string ("Filename" , verneed . vn_file . get (endian) , verneed . file (endian , strings) ,) ; p . field ("AuxOffset" , verneed . vn_aux . get (endian)) ; p . field ("NextOffset" , verneed . vn_next . get (endian)) ; while let Some (Some (vernaux)) = vernauxs . next () . print_err (p) { p . group ("Aux" , | p | { p . field_hex ("Hash" , vernaux . vna_hash . get (endian)) ; p . field_hex ("Flags" , vernaux . vna_flags . get (endian)) ; p . flags (vernaux . vna_flags . get (endian) , 0 , FLAGS_VER_FLG) ; p . field ("Index" , vernaux . vna_other . get (endian)) ; p . field_string ("Name" , vernaux . vna_name . get (endian) , vernaux . name (endian , strings) ,) ; p . field ("NextOffset" , vernaux . vna_next . get (endian)) ; }) ; } }) ; } } }
};
}
