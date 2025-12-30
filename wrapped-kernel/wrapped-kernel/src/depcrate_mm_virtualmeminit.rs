// Generated macro for init (function)
macro_rules! Depcrate_mm_virtualmeminit {
() => {
// Module: crate::mm::virtualmem
// Provides: {"init"}
// Dependencies: {}
unsafe fn init () { let range = PageRange :: new (kernel_heap_end () . as_usize () . div_ceil (2) , kernel_heap_end () . as_usize () + 1 ,) . unwrap () ; unsafe { PageAlloc :: deallocate (range) ; } }
};
}
