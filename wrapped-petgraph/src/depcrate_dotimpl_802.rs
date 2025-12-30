// Generated macro for impl_802 (impl)
macro_rules! Depcrate_dotimpl_802 {
() => {
// Module: crate::dot
// Provides: {"impl_802"}
// Dependencies: {}
impl < T > fmt :: Display for Escaped < T > where T : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if f . alternate () { writeln ! (& mut Escaper (f) , "{:#}" , & self . 0) } else { write ! (& mut Escaper (f) , "{}" , & self . 0) } } }
};
}
