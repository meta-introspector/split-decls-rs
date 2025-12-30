// Generated macro for impl_35 (impl)
macro_rules! Depcrate_singleimpl_35 {
() => {
// Module: crate::single
// Provides: {"impl_35"}
// Dependencies: {}
impl < T > Drop for Single < T > { fn drop (& mut self) { let Self { state , slot } = self ; state . with_mut (| state | { if * state & PUSHED != 0 { slot . with_mut (| slot | unsafe { let value = & mut * slot ; value . as_mut_ptr () . drop_in_place () ; }) ; } }) ; } }
};
}
