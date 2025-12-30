// Generated macro for impl_204 (impl)
macro_rules! Depcrate_internalsimpl_204 {
() => {
// Module: crate::internals
// Provides: {"impl_204"}
// Dependencies: {}
impl Writeable for FullMappingResult < '_ > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { match * self { FullMappingResult :: CodePoint (c) => sink . write_char (c) , FullMappingResult :: String (s) => sink . write_str (s) , FullMappingResult :: Remove => Ok (()) , } } }
};
}
