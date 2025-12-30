// Generated macro for macro_1074 (macro)
macro_rules! Depcrate_castsmacro_1074 {
() => {
// Module: crate::casts
// Provides: {"macro_1074"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions where a character literal is cast"] # [doc = " to `u8` and suggests using a byte literal instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In general, casting values to smaller types is"] # [doc = " error-prone and should be avoided where possible. In the particular case of"] # [doc = " converting a character literal to `u8`, it is easy to avoid by just using a"] # [doc = " byte literal instead. As an added bonus, `b'a'` is also slightly shorter"] # [doc = " than `'a' as u8`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " 'x' as u8"] # [doc = " ```"] # [doc = ""] # [doc = " A better version, using the byte literal:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " b'x'"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CHAR_LIT_AS_U8 , complexity , "casting a character literal to `u8` truncates" }
};
}
