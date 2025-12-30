// Generated macro for macro_125 (macro)
macro_rules! Depcrate_threadmacro_125 {
() => {
// Module: crate::thread
// Provides: {"macro_125"}
// Dependencies: {}
option ! { deallocatedp [str : b"thread.deallocatedp\0" , non_str : 2] => * mut u64 | ops : | docs : # [doc = " Access to the total number of bytes deallocated by the current thread."] # [doc = ""] # [doc = " The `read` method doesn't return the value directly, but actually a"] # [doc = " pointer to the value. This allows for very fast repeated lookup, since"] # [doc = " there is no function call overhead. The pointer type cannot be sent to"] # [doc = " other threads, but [`deallocatedp::read`] can be called on different"] # [doc = " threads and will return the appropriate pointer for each of them."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::thread;"] # [doc = " let deallocated = thread::deallocatedp::mib().unwrap();"] # [doc = " let deallocated = deallocated.read().unwrap();"] # [doc = ""] # [doc = " let a = deallocated.get();"] # [doc = " let buf = vec![0; 1024 * 1024];"] # [doc = " let b = deallocated.get();"] # [doc = " drop(buf);"] # [doc = " let c = deallocated.get();"] # [doc = ""] # [doc = " assert_eq!(a, b);"] # [doc = " assert!(b < c);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`deallocatedp`]."] }
};
}
