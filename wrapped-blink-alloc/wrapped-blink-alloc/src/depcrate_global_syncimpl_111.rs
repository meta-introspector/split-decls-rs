// Generated macro for impl_111 (impl)
macro_rules! Depcrate_global_syncimpl_111 {
() => {
// Module: crate::global::sync
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (feature = "std")] impl GlobalBlinkAlloc < std :: alloc :: System > { # [doc = " Create a new [`GlobalBlinkAlloc`]."] # [doc = ""] # [doc = " Const function can be used to initialize a static variable."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use blink_alloc::GlobalBlinkAlloc;"] # [doc = ""] # [doc = " #[global_allocator]"] # [doc = " static GLOBAL_ALLOC: GlobalBlinkAlloc = GlobalBlinkAlloc::new();"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _ = Box::new(42);"] # [doc = "     let _ = vec![1, 2, 3];"] # [doc = " }"] # [doc = " ```"] pub const fn new () -> Self { GlobalBlinkAlloc :: new_in (std :: alloc :: System) } # [doc = " Create a new [`GlobalBlinkAlloc`]."] # [doc = ""] # [doc = " This method allows to specify initial chunk size."] # [doc = ""] # [doc = " Const function can be used to initialize a static variable."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use blink_alloc::GlobalBlinkAlloc;"] # [doc = ""] # [doc = " #[global_allocator]"] # [doc = " static GLOBAL_ALLOC: GlobalBlinkAlloc = GlobalBlinkAlloc::new();"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _ = Box::new(42);"] # [doc = "     let _ = vec![1, 2, 3];"] # [doc = " }"] # [doc = " ```"] pub const fn with_chunk_size (chunk_size : usize) -> Self { GlobalBlinkAlloc :: with_chunk_size_in (chunk_size , std :: alloc :: System) } }
};
}
