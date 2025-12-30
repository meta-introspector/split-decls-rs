// Generated macro for memusage_linux (function)
macro_rules! Depcrate_memory_usagememusage_linux {
() => {
// Module: crate::memory_usage
// Provides: {"memusage_linux"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , target_env = "gnu" , not (feature = "jemalloc")))] fn memusage_linux () -> MemoryUsage { use std :: sync :: atomic :: { AtomicUsize , Ordering } ; static MALLINFO2 : AtomicUsize = AtomicUsize :: new (1) ; let mut mallinfo2 = MALLINFO2 . load (Ordering :: Relaxed) ; if mallinfo2 == 1 { mallinfo2 = unsafe { libc :: dlsym (libc :: RTLD_DEFAULT , c"mallinfo2" . as_ptr ()) } as usize ; MALLINFO2 . store (mallinfo2 , Ordering :: Relaxed) ; } if mallinfo2 == 0 { let alloc = unsafe { libc :: mallinfo () } . uordblks as isize ; MemoryUsage { allocated : Bytes (alloc) } } else { let mallinfo2 : extern "C" fn () -> libc :: mallinfo2 = unsafe { std :: mem :: transmute (mallinfo2) } ; let alloc = mallinfo2 () . uordblks as isize ; MemoryUsage { allocated : Bytes (alloc) } } }
};
}
