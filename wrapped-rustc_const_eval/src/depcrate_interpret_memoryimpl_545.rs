// Generated macro for impl_545 (impl)
macro_rules! Depcrate_interpret_memoryimpl_545 {
() => {
// Module: crate::interpret::memory
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'tcx , M : Machine < 'tcx > > Memory < 'tcx , M > { pub fn new () -> Self { Memory { alloc_map : M :: MemoryMap :: default () , extra_fn_ptr_map : FxIndexMap :: default () , dead_alloc_map : FxIndexMap :: default () , validation_in_progress : Cell :: new (false) , } } # [doc = " This is used by [priroda](https://github.com/oli-obk/priroda)"] pub fn alloc_map (& self) -> & M :: MemoryMap { & self . alloc_map } }
};
}
