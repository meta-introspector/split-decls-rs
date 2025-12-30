// Generated macro for macro_10107 (macro)
macro_rules! Depcrate_transmutemacro_10107 {
() => {
// Module: crate::transmute
// Provides: {"macro_10107"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes from a pointer to a pointer, or"] # [doc = " from a reference to a reference."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Transmutes are dangerous, and these can instead be"] # [doc = " written as casts."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let ptr = &1u32 as *const u32;"] # [doc = " unsafe {"] # [doc = "     // pointer-to-pointer transmute"] # [doc = "     let _: *const f32 = std::mem::transmute(ptr);"] # [doc = "     // ref-ref transmute"] # [doc = "     let _: &f32 = std::mem::transmute(&1u32);"] # [doc = " }"] # [doc = " // These can be respectively written:"] # [doc = " let _ = ptr as *const f32;"] # [doc = " let _ = unsafe{ &*(&1u32 as *const u32 as *const f32) };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TRANSMUTE_PTR_TO_PTR , pedantic , "transmutes from a pointer to a pointer / a reference to a reference" }
};
}
