// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl fmt :: Debug for SharedLibraryId { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let name = match * self { SharedLibraryId :: Uuid (..) => "Uuid" , SharedLibraryId :: GnuBuildId (..) => "GnuBuildId" , SharedLibraryId :: PeSignature (..) => "PeSignature" , SharedLibraryId :: PdbSignature (..) => "PdbSignature" , } ; write ! (f , "{}(\"{}\")" , name , self) } }
};
}
