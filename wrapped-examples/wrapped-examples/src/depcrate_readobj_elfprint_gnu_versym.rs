// Generated macro for print_gnu_versym (function)
macro_rules! Depcrate_readobj_elfprint_gnu_versym {
() => {
// Module: crate::readobj::elf
// Provides: {"print_gnu_versym"}
// Dependencies: {}
fn print_gnu_versym < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . elf_versions { return ; } if let Some (Some ((syms , _link))) = section . gnu_versym (endian , data) . print_err (p) { let versions = sections . versions (endian , data) . print_err (p) . flatten () ; for (index , sym) in syms . iter () . enumerate () { let version_index = VersionIndex (sym . 0 . get (endian)) ; p . group ("VersionSymbol" , | p | { p . field ("Index" , index) ; print_version (p , versions . as_ref () , version_index) ; }) ; } } }
};
}
