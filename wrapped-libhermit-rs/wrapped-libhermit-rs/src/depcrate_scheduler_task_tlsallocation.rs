// Generated macro for allocation (module)
macro_rules! Depcrate_scheduler_task_tlsallocation {
() => {
// Module: crate::scheduler::task::tls
// Provides: {"allocation"}
// Dependencies: {}
mod allocation { use core :: alloc :: Layout ; use core :: mem :: MaybeUninit ; use core :: slice ; pub struct Allocation { ptr : * mut u8 , layout : Layout , } impl Allocation { pub fn new (layout : Layout) -> Option < Self > { let ptr = unsafe { :: alloc :: alloc :: alloc (layout) } ; if ptr . is_null () { return None ; } Some (Self { ptr , layout }) } pub fn as_mut_ptr (& mut self) -> * mut u8 { self . ptr } pub fn as_mut_slice (& mut self) -> & mut [MaybeUninit < u8 >] { unsafe { slice :: from_raw_parts_mut (self . ptr . cast () , self . layout . size ()) } } } impl Drop for Allocation { fn drop (& mut self) { unsafe { :: alloc :: alloc :: dealloc (self . ptr , self . layout) ; } } } }
};
}
