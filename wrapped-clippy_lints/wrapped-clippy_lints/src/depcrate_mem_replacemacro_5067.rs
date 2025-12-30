// Generated macro for macro_5067 (macro)
macro_rules! Depcrate_mem_replacemacro_5067 {
() => {
// Module: crate::mem_replace
// Provides: {"macro_5067"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `mem::replace(&mut _, mem::uninitialized())`"] # [doc = " and `mem::replace(&mut _, mem::zeroed())`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will lead to undefined behavior even if the"] # [doc = " value is overwritten later, because the uninitialized value may be"] # [doc = " observed in the case of a panic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::mem;"] # [doc = "# fn may_panic(v: Vec<i32>) -> Vec<i32> { v }"] # [doc = ""] # [doc = " #[allow(deprecated, invalid_value)]"] # [doc = " fn myfunc (v: &mut Vec<i32>) {"] # [doc = "     let taken_v = unsafe { mem::replace(v, mem::uninitialized()) };"] # [doc = "     let new_v = may_panic(taken_v); // undefined behavior on panic"] # [doc = "     mem::forget(mem::replace(v, new_v));"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The [take_mut](https://docs.rs/take_mut) crate offers a sound solution,"] # [doc = " at the cost of either lazily creating a replacement value or aborting"] # [doc = " on panic, to ensure that the uninitialized value cannot be observed."] # [clippy :: version = "1.39.0"] pub MEM_REPLACE_WITH_UNINIT , correctness , "`mem::replace(&mut _, mem::uninitialized())` or `mem::replace(&mut _, mem::zeroed())`" }
};
}
