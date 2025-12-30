// Generated macro for impl_161 (impl)
macro_rules! Depcrate_registryimpl_161 {
() => {
// Module: crate::registry
// Provides: {"impl_161"}
// Dependencies: {}
impl ThreadBuilder { # [doc = " Gets the index of this thread in the pool, within `0..num_threads`."] pub fn index (& self) -> usize { self . index } # [doc = " Gets the string that was specified by `ThreadPoolBuilder::name()`."] pub fn name (& self) -> Option < & str > { self . name . as_deref () } # [doc = " Gets the value that was specified by `ThreadPoolBuilder::stack_size()`."] pub fn stack_size (& self) -> Option < usize > { self . stack_size } # [doc = " Executes the main loop for this thread. This will not return until the"] # [doc = " thread pool is dropped."] pub fn run (self) { unsafe { main_loop (self) } } }
};
}
