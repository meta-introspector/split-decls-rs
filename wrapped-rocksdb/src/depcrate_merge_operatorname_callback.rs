// Generated macro for name_callback (function)
macro_rules! Depcrate_merge_operatorname_callback {
() => {
// Module: crate::merge_operator
// Provides: {"name_callback"}
// Dependencies: {}
pub unsafe extern "C" fn name_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void ,) -> * const c_char { let cb = unsafe { & mut * (raw_cb as * mut MergeOperatorCallback < F , PF >) } ; cb . name . as_ptr () }
};
}
