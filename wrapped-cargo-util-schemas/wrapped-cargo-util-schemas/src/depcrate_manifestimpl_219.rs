// Generated macro for impl_219 (impl)
macro_rules! Depcrate_manifestimpl_219 {
() => {
// Module: crate::manifest
// Provides: {"impl_219"}
// Dependencies: {}
impl InheritableLints { pub fn normalized (& self) -> Result < & TomlLints , UnresolvedError > { if self . workspace { Err (UnresolvedError) } else { Ok (& self . lints) } } }
};
}
