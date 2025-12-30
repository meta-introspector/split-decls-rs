// Generated macro for atomic_load_aligned (function)
macro_rules! Depcrate_arm_linuxatomic_load_aligned {
() => {
// Module: crate::arm_linux
// Provides: {"atomic_load_aligned"}
// Dependencies: {}
# [doc = " Performs a relaxed atomic load of 4 bytes at `ptr`. Some of the bytes are allowed to be out of"] # [doc = " bounds as long as `size_of::<T>()` bytes are in bounds."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `ptr` must be 4-aligned."] # [doc = " - `size_of::<T>()` must be at most 4."] # [doc = " - if `size_of::<T>() == 1`, `ptr` or `ptr` offset by 1, 2 or 3 bytes must be valid for a relaxed"] # [doc = "   atomic read of 1 byte."] # [doc = " - if `size_of::<T>() == 2`, `ptr` or `ptr` offset by 2 bytes must be valid for a relaxed atomic"] # [doc = "   read of 2 bytes."] # [doc = " - if `size_of::<T>() == 4`, `ptr` must be valid for a relaxed atomic read of 4 bytes."] unsafe fn atomic_load_aligned < T > (ptr : * mut u32) -> u32 { const { assert ! (size_of ::< T > () <= 4) } ; if size_of :: < T > () == 4 { unsafe { AtomicU32 :: from_ptr (ptr) . load (Ordering :: Relaxed) } } else { unsafe { let res : u32 ; arch :: asm ! ("ldr {res}, [{ptr}]" , ptr = in (reg) ptr , res = lateout (reg) res , options (nostack , preserves_flags , readonly)) ; res } } }
};
}
