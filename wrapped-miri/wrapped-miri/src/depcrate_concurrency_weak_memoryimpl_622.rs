// Generated macro for impl_622 (impl)
macro_rules! Depcrate_concurrency_weak_memoryimpl_622 {
() => {
// Module: crate::concurrency::weak_memory
// Provides: {"impl_622"}
// Dependencies: {}
impl StoreElement { # [doc = " ATOMIC LOAD IMPL in the paper"] # [doc = " Unlike the operational semantics in the paper, we don't need to keep track"] # [doc = " of the thread timestamp for every single load. Keeping track of the first (smallest)"] # [doc = " timestamp of each thread that has loaded from a store is sufficient: if the earliest"] # [doc = " load of another thread happens before the current one, then we must stop searching the store"] # [doc = " buffer regardless of subsequent loads by the same thread; if the earliest load of another"] # [doc = " thread doesn't happen before the current one, then no subsequent load by the other thread"] # [doc = " can happen before the current one."] fn load_impl (& self , index : VectorIdx , clocks : & ThreadClockSet , is_seqcst : bool ,) -> Option < Scalar > { let mut load_info = self . load_info . borrow_mut () ; load_info . sc_loaded |= is_seqcst ; let _ = load_info . timestamps . try_insert (index , clocks . clock [index]) ; self . val } }
};
}
