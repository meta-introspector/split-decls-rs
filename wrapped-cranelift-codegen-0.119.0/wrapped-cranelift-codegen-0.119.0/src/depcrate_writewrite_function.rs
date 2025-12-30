// Generated macro for write_function (function)
macro_rules! Depcrate_writewrite_function {
() => {
// Module: crate::write
// Provides: {"write_function"}
// Dependencies: {}
# [doc = " Write `func` to `w` as equivalent text."] # [doc = " Use `isa` to emit ISA-dependent annotations."] pub fn write_function (w : & mut dyn Write , func : & Function) -> fmt :: Result { decorate_function (& mut PlainWriter , w , func) }
};
}
