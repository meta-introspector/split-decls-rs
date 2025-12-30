// Generated macro for AsmFileExt (enum)
macro_rules! DepcrateAsmFileExt {
() => {
// Module: crate
// Provides: {"AsmFileExt"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq)] enum AsmFileExt { # [doc = " `.asm` files. On MSVC targets, we assume these should be passed to MASM"] # [doc = " (`ml{,64}.exe`)."] DotAsm , # [doc = " `.s` or `.S` files, which do not have the special handling on MSVC targets."] DotS , }
};
}
