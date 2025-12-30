// Generated macro for destructor_callback (function)
macro_rules! Depcrate_merge_operatordestructor_callback {
() => {
// Module: crate::merge_operator
// Provides: {"destructor_callback"}
// Dependencies: {}
pub unsafe extern "C" fn destructor_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut MergeOperatorCallback < F , PF >) }) ; }
};
}
