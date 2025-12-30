// Generated macro for impl_384 (impl)
macro_rules! Depcrate_coherence_builtinimpl_384 {
() => {
// Module: crate::coherence::builtin
// Provides: {"impl_384"}
// Dependencies: {}
impl < 'tcx > Checker < 'tcx > { fn check (& self , trait_def_id : Option < DefId > , f : impl FnOnce (& Self) -> Result < () , ErrorGuaranteed > ,) -> Result < () , ErrorGuaranteed > { if Some (self . trait_def_id) == trait_def_id { f (self) } else { Ok (()) } } }
};
}
