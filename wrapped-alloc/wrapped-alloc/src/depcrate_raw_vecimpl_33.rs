// Generated macro for impl_33 (impl)
macro_rules! Depcrate_raw_vecimpl_33 {
() => {
// Module: crate::raw_vec
// Provides: {"impl_33"}
// Dependencies: {}
impl RawVecInner < Global > { # [cfg (not (any (no_global_oom_handling , test)))] # [must_use] # [inline] # [track_caller] fn with_capacity (capacity : usize , elem_layout : Layout) -> Self { match Self :: try_allocate_in (capacity , AllocInit :: Uninitialized , Global , elem_layout) { Ok (res) => res , Err (err) => handle_error (err) , } } }
};
}
