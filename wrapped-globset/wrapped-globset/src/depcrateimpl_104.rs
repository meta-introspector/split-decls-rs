// Generated macro for impl_104 (impl)
macro_rules! Depcrateimpl_104 {
() => {
// Module: crate
// Provides: {"impl_104"}
// Dependencies: {}
impl RequiredExtensionStrategyBuilder { fn new () -> RequiredExtensionStrategyBuilder { RequiredExtensionStrategyBuilder (fnv :: HashMap :: default ()) } fn add (& mut self , global_index : usize , ext : String , regex : String) { self . 0 . entry (ext . into_bytes ()) . or_insert (vec ! []) . push ((global_index , regex)) ; } fn build (self) -> Result < RequiredExtensionStrategy , Error > { let mut exts = fnv :: HashMap :: default () ; for (ext , regexes) in self . 0 . into_iter () { exts . insert (ext . clone () , vec ! []) ; for (global_index , regex) in regexes { let compiled = new_regex (& regex) ? ; exts . get_mut (& ext) . unwrap () . push ((global_index , compiled)) ; } } Ok (RequiredExtensionStrategy (exts)) } }
};
}
