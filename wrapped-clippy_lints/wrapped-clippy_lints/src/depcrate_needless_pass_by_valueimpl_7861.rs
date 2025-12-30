// Generated macro for impl_7861 (impl)
macro_rules! Depcrate_needless_pass_by_valueimpl_7861 {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"impl_7861"}
// Dependencies: {}
impl MovedVariablesCtxt { fn move_common (& mut self , cmt : & euv :: PlaceWithHirId < '_ >) { if let euv :: PlaceBase :: Local (vid) = cmt . place . base { self . moved_vars . insert (vid) ; } } }
};
}
