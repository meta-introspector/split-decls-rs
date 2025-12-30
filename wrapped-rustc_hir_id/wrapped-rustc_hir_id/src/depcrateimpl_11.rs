// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl rustc_index :: Idx for OwnerId { # [inline] fn new (idx : usize) -> Self { OwnerId { def_id : LocalDefId { local_def_index : DefIndex :: from_usize (idx) } } } # [inline] fn index (self) -> usize { self . def_id . local_def_index . as_usize () } }
};
}
