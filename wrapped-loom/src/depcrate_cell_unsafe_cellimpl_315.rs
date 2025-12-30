// Generated macro for impl_315 (impl)
macro_rules! Depcrate_cell_unsafe_cellimpl_315 {
() => {
// Module: crate::cell::unsafe_cell
// Provides: {"impl_315"}
// Dependencies: {}
impl < T > UnsafeCell < T > { # [doc = " Constructs a new instance of `UnsafeCell` which will wrap the specified value."] # [track_caller] pub fn new (data : T) -> UnsafeCell < T > { let state = rt :: Cell :: new (location ! ()) ; UnsafeCell { state , data : std :: cell :: UnsafeCell :: new (data) , } } # [doc = " Unwraps the value."] pub fn into_inner (self) -> T { self . data . into_inner () } }
};
}
