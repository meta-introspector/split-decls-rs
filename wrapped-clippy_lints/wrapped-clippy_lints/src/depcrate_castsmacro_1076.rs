// Generated macro for macro_1076 (macro)
macro_rules! Depcrate_castsmacro_1076 {
() => {
// Module: crate::casts
// Provides: {"macro_1076"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `as` casts between raw pointers that change their constness, namely `*const T` to"] # [doc = " `*mut T` and `*mut T` to `*const T`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Though `as` casts between raw pointers are not terrible, `pointer::cast_mut` and"] # [doc = " `pointer::cast_const` are safer because they cannot accidentally cast the pointer to another"] # [doc = " type. Or, when null pointers are involved, `null()` and `null_mut()` can be used directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let ptr: *const u32 = &42_u32;"] # [doc = " let mut_ptr = ptr as *mut u32;"] # [doc = " let ptr = mut_ptr as *const u32;"] # [doc = " let ptr1 = std::ptr::null::<u32>() as *mut u32;"] # [doc = " let ptr2 = std::ptr::null_mut::<u32>() as *const u32;"] # [doc = " let ptr3 = std::ptr::null::<u32>().cast_mut();"] # [doc = " let ptr4 = std::ptr::null_mut::<u32>().cast_const();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let ptr: *const u32 = &42_u32;"] # [doc = " let mut_ptr = ptr.cast_mut();"] # [doc = " let ptr = mut_ptr.cast_const();"] # [doc = " let ptr1 = std::ptr::null_mut::<u32>();"] # [doc = " let ptr2 = std::ptr::null::<u32>();"] # [doc = " let ptr3 = std::ptr::null_mut::<u32>();"] # [doc = " let ptr4 = std::ptr::null::<u32>();"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub PTR_CAST_CONSTNESS , pedantic , "casting using `as` on raw pointers to change constness when specialized methods apply" }
};
}
