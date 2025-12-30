// Generated macro for impl_304 (impl)
macro_rules! Depcrate_hirimpl_304 {
() => {
// Module: crate::hir
// Provides: {"impl_304"}
// Dependencies: {}
impl fmt :: Display for CoroutineDesugaring { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineDesugaring :: Async => { if f . alternate () { f . write_str ("`async` ") ? ; } else { f . write_str ("async ") ? } } CoroutineDesugaring :: Gen => { if f . alternate () { f . write_str ("`gen` ") ? ; } else { f . write_str ("gen ") ? } } CoroutineDesugaring :: AsyncGen => { if f . alternate () { f . write_str ("`async gen` ") ? ; } else { f . write_str ("async gen ") ? } } } Ok (()) } }
};
}
