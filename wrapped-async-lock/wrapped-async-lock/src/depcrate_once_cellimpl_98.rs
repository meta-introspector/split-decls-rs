// Generated macro for impl_98 (impl)
macro_rules! Depcrate_once_cellimpl_98 {
() => {
// Module: crate::once_cell
// Provides: {"impl_98"}
// Dependencies: {}
impl < T > Drop for OnceCell < T > { fn drop (& mut self) { self . state . with_mut (| state | { if State :: from (* state) == State :: Initialized { unsafe { self . value . get () . cast :: < T > () . drop_in_place () } } }) ; } }
};
}
