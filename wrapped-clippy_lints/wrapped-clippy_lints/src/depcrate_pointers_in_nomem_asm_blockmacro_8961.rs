// Generated macro for macro_8961 (macro)
macro_rules! Depcrate_pointers_in_nomem_asm_blockmacro_8961 {
() => {
// Module: crate::pointers_in_nomem_asm_block
// Provides: {"macro_8961"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if any pointer is being passed to an asm! block with `nomem` option."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `nomem` forbids any reads or writes to memory and passing a pointer suggests"] # [doc = " that either of those will happen."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn f(p: *mut u32) {"] # [doc = "     unsafe { core::arch::asm!(\"mov [{p}], 42\", p = in(reg) p, options(nomem, nostack)); }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn f(p: *mut u32) {"] # [doc = "     unsafe { core::arch::asm!(\"mov [{p}], 42\", p = in(reg) p, options(nostack)); }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub POINTERS_IN_NOMEM_ASM_BLOCK , suspicious , "pointers in nomem asm block" }
};
}
