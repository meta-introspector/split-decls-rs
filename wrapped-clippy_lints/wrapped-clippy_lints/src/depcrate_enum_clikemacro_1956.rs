// Generated macro for macro_1956 (macro)
macro_rules! Depcrate_enum_clikemacro_1956 {
() => {
// Module: crate::enum_clike
// Provides: {"macro_1956"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for C-like enumerations that are"] # [doc = " `repr(isize/usize)` and have values that don't fit into an `i32`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will truncate the variant value on 32 bit"] # [doc = " architectures, but works fine on 64 bit."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # #[cfg(target_pointer_width = \"64\")]"] # [doc = " #[repr(usize)]"] # [doc = " enum NonPortable {"] # [doc = "     X = 0x1_0000_0000,"] # [doc = "     Y = 0,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ENUM_CLIKE_UNPORTABLE_VARIANT , correctness , "C-like enums that are `repr(isize/usize)` and have values that don't fit into an `i32`" }
};
}
