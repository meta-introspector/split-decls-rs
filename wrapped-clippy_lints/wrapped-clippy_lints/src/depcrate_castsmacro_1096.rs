// Generated macro for macro_1096 (macro)
macro_rules! Depcrate_castsmacro_1096 {
() => {
// Module: crate::casts
// Provides: {"macro_1096"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for a raw slice being cast to a slice pointer"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can result in multiple `&mut` references to the same location when only a pointer is"] # [doc = " required."] # [doc = " `ptr::slice_from_raw_parts` is a safe alternative that doesn't require"] # [doc = " the same [safety requirements] to be upheld."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _: *const [u8] = std::slice::from_raw_parts(ptr, len) as *const _;"] # [doc = " let _: *mut [u8] = std::slice::from_raw_parts_mut(ptr, len) as *mut _;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let _: *const [u8] = std::ptr::slice_from_raw_parts(ptr, len);"] # [doc = " let _: *mut [u8] = std::ptr::slice_from_raw_parts_mut(ptr, len);"] # [doc = " ```"] # [doc = " [safety requirements]: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html#safety"] # [clippy :: version = "1.65.0"] pub CAST_SLICE_FROM_RAW_PARTS , suspicious , "casting a slice created from a pointer and length to a slice pointer" }
};
}
