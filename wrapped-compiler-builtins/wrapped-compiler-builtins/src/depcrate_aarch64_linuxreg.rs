// Generated macro for reg (macro)
macro_rules! Depcrate_aarch64_linuxreg {
() => {
// Module: crate::aarch64_linux
// Provides: {"reg"}
// Dependencies: {}
# [doc = " Given a byte size and a register number, return a register of the appropriate size."] # [doc = ""] # [doc = " See <https://developer.arm.com/documentation/102374/0101/Registers-in-AArch64---general-purpose-registers>."] # [rustfmt :: skip] macro_rules ! reg { (1 , $ num : literal) => { concat ! ("w" , $ num) } ; (2 , $ num : literal) => { concat ! ("w" , $ num) } ; (4 , $ num : literal) => { concat ! ("w" , $ num) } ; (8 , $ num : literal) => { concat ! ("x" , $ num) } ; (16 , $ num : literal) => { concat ! ("x" , $ num) } ; }
};
}
