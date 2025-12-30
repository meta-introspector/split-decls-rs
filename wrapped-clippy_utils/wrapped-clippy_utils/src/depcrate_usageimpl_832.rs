// Generated macro for impl_832 (impl)
macro_rules! Depcrate_usageimpl_832 {
() => {
// Module: crate::usage
// Provides: {"impl_832"}
// Dependencies: {}
impl MutVarsDelegate { fn update (& mut self , cat : & PlaceWithHirId < '_ >) { match cat . place . base { PlaceBase :: Local (id) => { self . used_mutably . insert (id) ; } , PlaceBase :: Upvar (_) => { self . skip = true ; } , _ => { } , } } }
};
}
