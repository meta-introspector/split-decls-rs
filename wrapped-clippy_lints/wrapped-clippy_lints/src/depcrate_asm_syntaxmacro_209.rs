// Generated macro for macro_209 (macro)
macro_rules! Depcrate_asm_syntaxmacro_209 {
() => {
// Module: crate::asm_syntax
// Provides: {"macro_209"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of Intel x86 assembly syntax."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To enforce consistent use of AT&T x86 assembly syntax."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # #![feature(asm)]"] # [doc = " # #[cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))]"] # [doc = " # unsafe { let ptr = \"\".as_ptr();"] # [doc = " # use std::arch::asm;"] # [doc = " asm!(\"lea {}, [{}]\", lateout(reg) _, in(reg) ptr);"] # [doc = " # }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,no_run"] # [doc = " # #![feature(asm)]"] # [doc = " # #[cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))]"] # [doc = " # unsafe { let ptr = \"\".as_ptr();"] # [doc = " # use std::arch::asm;"] # [doc = " asm!(\"lea ({}), {}\", in(reg) ptr, lateout(reg) _, options(att_syntax));"] # [doc = " # }"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub INLINE_ASM_X86_INTEL_SYNTAX , restriction , "prefer AT&T x86 assembly syntax" }
};
}
