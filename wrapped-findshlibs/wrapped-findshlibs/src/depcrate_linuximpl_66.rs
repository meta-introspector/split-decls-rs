// Generated macro for impl_66 (impl)
macro_rules! Depcrate_linuximpl_66 {
() => {
// Module: crate::linux
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > fmt :: Debug for SharedLibrary < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "SharedLibrary {{ size: {:?}, addr: {:?}, " , self . size , self . addr) ? ; write ! (f , "name: {:?}, headers: [" , self . name) ? ; let l = self . headers . len () ; self . headers [.. (l - 1)] . into_iter () . map (| phdr | write ! (f , "{:?}, " , & DebugPhdr (phdr))) . collect :: < fmt :: Result > () ? ; write ! (f , "{:?}" , & DebugPhdr (& self . headers [l - 1])) ? ; write ! (f , "] }}") } }
};
}
