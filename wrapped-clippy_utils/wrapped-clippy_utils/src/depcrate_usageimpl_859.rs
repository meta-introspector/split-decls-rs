// Generated macro for impl_859 (impl)
macro_rules! Depcrate_usageimpl_859 {
() => {
// Module: crate::usage
// Provides: {"impl_859"}
// Dependencies: {}
impl MutVarsDelegate { fn update (& mut self , cat : & PlaceWithHirId < '_ >) { match cat . place . base { PlaceBase :: Local (id) => { self . used_mutably . insert (id) ; } , PlaceBase :: Upvar (_) => { self . skip = true ; } , _ => { } , } } }
};
}
