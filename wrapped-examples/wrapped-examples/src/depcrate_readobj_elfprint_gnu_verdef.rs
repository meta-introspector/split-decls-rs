// Generated macro for print_gnu_verdef (function)
macro_rules! Depcrate_readobj_elfprint_gnu_verdef {
() => {
// Module: crate::readobj::elf
// Provides: {"print_gnu_verdef"}
// Dependencies: {}
fn print_gnu_verdef < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . elf_versions { return ; } if let Some (Some ((mut verdefs , link))) = section . gnu_verdef (endian , data) . print_err (p) { let strings = sections . strings (endian , data , link) . unwrap_or_default () ; while let Some (Some ((verdef , mut verdauxs))) = verdefs . next () . print_err (p) { p . group ("VersionDefinition" , | p | { p . field ("Version" , verdef . vd_version . get (endian)) ; p . field_hex ("Flags" , verdef . vd_flags . get (endian)) ; p . flags (verdef . vd_flags . get (endian) , 0 , FLAGS_VER_FLG) ; p . field ("Index" , verdef . vd_ndx . get (endian)) ; p . field ("AuxCount" , verdef . vd_cnt . get (endian)) ; p . field_hex ("Hash" , verdef . vd_hash . get (endian)) ; p . field ("AuxOffset" , verdef . vd_aux . get (endian)) ; p . field ("NextOffset" , verdef . vd_next . get (endian)) ; while let Some (Some (verdaux)) = verdauxs . next () . print_err (p) { p . group ("Aux" , | p | { p . field_string ("Name" , verdaux . vda_name . get (endian) , verdaux . name (endian , strings) ,) ; p . field ("NextOffset" , verdaux . vda_next . get (endian)) ; }) ; } }) ; } } }
};
}
