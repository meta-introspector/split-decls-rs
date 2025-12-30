// Generated macro for macro_7611 (macro)
macro_rules! Depcrate_needless_arbitrary_self_typemacro_7611 {
() => {
// Module: crate::needless_arbitrary_self_type
// Provides: {"macro_7611"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " The lint checks for `self` in fn parameters that"] # [doc = " specify the `Self`-type explicitly"] # [doc = " ### Why is this bad?"] # [doc = " Increases the amount and decreases the readability of code"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum ValType {"] # [doc = "     I32,"] # [doc = "     I64,"] # [doc = "     F32,"] # [doc = "     F64,"] # [doc = " }"] # [doc = ""] # [doc = " impl ValType {"] # [doc = "     pub fn bytes(self: Self) -> usize {"] # [doc = "         match self {"] # [doc = "             Self::I32 | Self::F32 => 4,"] # [doc = "             Self::I64 | Self::F64 => 8,"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be rewritten as"] # [doc = ""] # [doc = " ```no_run"] # [doc = " enum ValType {"] # [doc = "     I32,"] # [doc = "     I64,"] # [doc = "     F32,"] # [doc = "     F64,"] # [doc = " }"] # [doc = ""] # [doc = " impl ValType {"] # [doc = "     pub fn bytes(self) -> usize {"] # [doc = "         match self {"] # [doc = "             Self::I32 | Self::F32 => 4,"] # [doc = "             Self::I64 | Self::F64 => 8,"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub NEEDLESS_ARBITRARY_SELF_TYPE , complexity , "type of `self` parameter is already by default `Self`" }
};
}
