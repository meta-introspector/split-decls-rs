// Generated macro for impl_1714 (impl)
macro_rules! Depcrate_vec_extract_ifimpl_1714 {
() => {
// Module: crate::vec::extract_if
// Provides: {"impl_1714"}
// Dependencies: {}
# [stable (feature = "extract_if" , since = "1.87.0")] impl < T , F , A : Allocator > Drop for ExtractIf < '_ , T , F , A > { fn drop (& mut self) { unsafe { if self . idx < self . old_len && self . del > 0 { let ptr = self . vec . as_mut_ptr () ; let src = ptr . add (self . idx) ; let dst = src . sub (self . del) ; let tail_len = self . old_len - self . idx ; src . copy_to (dst , tail_len) ; } self . vec . set_len (self . old_len - self . del) ; } } }
};
}
