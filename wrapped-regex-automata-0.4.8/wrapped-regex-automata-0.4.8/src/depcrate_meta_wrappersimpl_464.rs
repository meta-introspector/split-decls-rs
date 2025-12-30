// Generated macro for impl_464 (impl)
macro_rules! Depcrate_meta_wrappersimpl_464 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_464"}
// Dependencies: {}
impl PikeVM { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA ,) -> Result < PikeVM , BuildError > { PikeVMEngine :: new (info , pre , nfa) . map (PikeVM) } pub (crate) fn create_cache (& self) -> PikeVMCache { PikeVMCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self) -> & PikeVMEngine { & self . 0 } }
};
}
