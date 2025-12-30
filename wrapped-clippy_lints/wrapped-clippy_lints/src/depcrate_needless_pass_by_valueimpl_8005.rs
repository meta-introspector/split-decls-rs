// Generated macro for impl_8005 (impl)
macro_rules! Depcrate_needless_pass_by_valueimpl_8005 {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"impl_8005"}
// Dependencies: {}
impl MovedVariablesCtxt { fn move_common (& mut self , cmt : & euv :: PlaceWithHirId < '_ >) { if let euv :: PlaceBase :: Local (vid) = cmt . place . base { self . moved_vars . insert (vid) ; } } }
};
}
