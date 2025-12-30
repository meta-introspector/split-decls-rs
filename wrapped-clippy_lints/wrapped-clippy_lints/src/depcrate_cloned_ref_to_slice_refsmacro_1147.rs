// Generated macro for macro_1147 (macro)
macro_rules! Depcrate_cloned_ref_to_slice_refsmacro_1147 {
() => {
// Module: crate::cloned_ref_to_slice_refs
// Provides: {"macro_1147"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for slice references with cloned references such as `&[f.clone()]`."] # [doc = ""] # [doc = " ### Why is this bad"] # [doc = ""] # [doc = " A reference does not need to be owned in order to be used as a slice."] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " This lint does not know whether or not a clone implementation has side effects."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let data = 10;"] # [doc = " let data_ref = &data;"] # [doc = " take_slice(&[data_ref.clone()]);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " use std::slice;"] # [doc = " let data = 10;"] # [doc = " let data_ref = &data;"] # [doc = " take_slice(slice::from_ref(data_ref));"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub CLONED_REF_TO_SLICE_REFS , perf , "cloning a reference for slice references" }
};
}
