// Generated macro for impl_847 (impl)
macro_rules! Depcrate_read_macho_sectionimpl_847 {
() => {
// Module: crate::read::macho::section
// Provides: {"impl_847"}
// Dependencies: {}
impl < 'data , Mach : MachHeader , R : ReadRef < 'data > > MachOSectionInternal < 'data , Mach , R > { pub (super) fn parse (index : SectionIndex , section : & 'data Mach :: Section , data : R) -> Self { let kind = match (section . segment_name () , section . name ()) { (b"__TEXT" , b"__text") => SectionKind :: Text , (b"__TEXT" , b"__const") => SectionKind :: ReadOnlyData , (b"__TEXT" , b"__cstring") => SectionKind :: ReadOnlyString , (b"__TEXT" , b"__literal4") => SectionKind :: ReadOnlyData , (b"__TEXT" , b"__literal8") => SectionKind :: ReadOnlyData , (b"__TEXT" , b"__literal16") => SectionKind :: ReadOnlyData , (b"__TEXT" , b"__eh_frame") => SectionKind :: ReadOnlyData , (b"__TEXT" , b"__gcc_except_tab") => SectionKind :: ReadOnlyData , (b"__DATA" , b"__data") => SectionKind :: Data , (b"__DATA" , b"__const") => SectionKind :: ReadOnlyData , (b"__DATA" , b"__bss") => SectionKind :: UninitializedData , (b"__DATA" , b"__common") => SectionKind :: Common , (b"__DATA" , b"__thread_data") => SectionKind :: Tls , (b"__DATA" , b"__thread_bss") => SectionKind :: UninitializedTls , (b"__DATA" , b"__thread_vars") => SectionKind :: TlsVariables , (b"__DWARF" , _) => SectionKind :: Debug , _ => SectionKind :: Unknown , } ; MachOSectionInternal { index , kind , section , data , } } }
};
}
