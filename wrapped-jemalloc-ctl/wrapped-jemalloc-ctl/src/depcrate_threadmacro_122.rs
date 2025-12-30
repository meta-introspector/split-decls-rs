// Generated macro for macro_122 (macro)
macro_rules! Depcrate_threadmacro_122 {
() => {
// Module: crate::thread
// Provides: {"macro_122"}
// Dependencies: {}
option ! { allocatedp [str : b"thread.allocatedp\0" , non_str : 2] => * mut u64 | ops : | docs : # [doc = " Access to the total number of bytes allocated by the current thread."] # [doc = ""] # [doc = " Unlike [`crate::stats::allocated`], the value returned by this type is not the"] # [doc = " number of bytes *currently* allocated, but rather the number of bytes"] # [doc = " that have *ever* been allocated by this thread."] # [doc = ""] # [doc = " The `read` method doesn't return the value directly, but actually a"] # [doc = " pointer to the value. This allows for very fast repeated lookup, since"] # [doc = " there is no function call overhead. The pointer type cannot be sent to"] # [doc = " other threads, but `allocated::read` can be called on different threads"] # [doc = " and will return the appropriate pointer for each of them."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::thread;"] # [doc = " let allocated = thread::allocatedp::mib().unwrap();"] # [doc = " let allocated = allocated.read().unwrap();"] # [doc = ""] # [doc = " let a = allocated.get();"] # [doc = " let buf = vec![0; 1024 * 1024];"] # [doc = " let b = allocated.get();"] # [doc = " drop(    buf);"] # [doc = " let c = allocated.get();"] # [doc = ""] # [doc = " assert!(a < b);"] # [doc = " assert_eq!(b, c);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`allocatedp`]."] }
};
}
