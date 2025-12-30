// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl fmt :: Display for SharedLibraryId { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { SharedLibraryId :: Uuid (ref bytes) => { for (idx , byte) in bytes . iter () . enumerate () { if idx == 4 || idx == 6 || idx == 8 || idx == 10 { write ! (f , "-") ? ; } write ! (f , "{:02x}" , byte) ? ; } } SharedLibraryId :: GnuBuildId (ref bytes) => { for byte in bytes { write ! (f , "{:02x}" , byte) ? ; } } SharedLibraryId :: PeSignature (timestamp , size_of_image) => { write ! (f , "{:08X}{:x}" , timestamp , size_of_image) ? ; } SharedLibraryId :: PdbSignature (ref bytes , age) => { for (idx , byte) in bytes . iter () . enumerate () { if idx == 4 || idx == 6 || idx == 8 || idx == 10 { write ! (f , "-") ? ; } write ! (f , "{:02X}" , byte) ? ; } write ! (f , "{:x}" , age) ? ; } } Ok (()) } }
};
}
