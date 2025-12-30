// Generated macro for impl_103 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_103 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_103"}
// Dependencies: {}
impl LintConfig { fn new (builder : & Builder < '_ >) -> Self { match builder . config . cmd . clone () { Subcommand :: Clippy { allow , deny , warn , forbid , .. } => { Self { allow , warn , deny , forbid } } _ => unreachable ! ("LintConfig can only be called from `clippy` subcommands.") , } } fn merge (& self , other : & Self) -> Self { let merged = | self_attr : & [String] , other_attr : & [String] | -> Vec < String > { self_attr . iter () . cloned () . chain (other_attr . iter () . cloned ()) . collect () } ; Self { allow : merged (& self . allow , & other . allow) , warn : merged (& self . warn , & other . warn) , deny : merged (& self . deny , & other . deny) , forbid : merged (& self . forbid , & other . forbid) , } } }
};
}
