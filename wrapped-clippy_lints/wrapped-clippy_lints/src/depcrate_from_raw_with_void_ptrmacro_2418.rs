// Generated macro for macro_2418 (macro)
macro_rules! Depcrate_from_raw_with_void_ptrmacro_2418 {
() => {
// Module: crate::from_raw_with_void_ptr
// Provides: {"macro_2418"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if we're passing a `c_void` raw pointer to `{Box,Rc,Arc,Weak}::from_raw(_)`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " When dealing with `c_void` raw pointers in FFI, it is easy to run into the pitfall of calling `from_raw` with the `c_void` pointer."] # [doc = " The type signature of `Box::from_raw` is `fn from_raw(raw: *mut T) -> Box<T>`, so if you pass a `*mut c_void` you will get a `Box<c_void>` (and similarly for `Rc`, `Arc` and `Weak`)."] # [doc = " For this to be safe, `c_void` would need to have the same memory layout as the original type, which is often not the case."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::ffi::c_void;"] # [doc = " let ptr = Box::into_raw(Box::new(42usize)) as *mut c_void;"] # [doc = " let _ = unsafe { Box::from_raw(ptr) };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::ffi::c_void;"] # [doc = " # let ptr = Box::into_raw(Box::new(42usize)) as *mut c_void;"] # [doc = " let _ = unsafe { Box::from_raw(ptr as *mut usize) };"] # [doc = " ```"] # [doc = ""] # [clippy :: version = "1.67.0"] pub FROM_RAW_WITH_VOID_PTR , suspicious , "creating a `Box` from a void raw pointer" }
};
}
