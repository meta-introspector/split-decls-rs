// Generated macro for macro_9231 (macro)
macro_rules! Depcrate_read_zero_byte_vecmacro_9231 {
() => {
// Module: crate::read_zero_byte_vec
// Provides: {"macro_9231"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint catches reads into a zero-length `Vec`."] # [doc = " Especially in the case of a call to `with_capacity`, this lint warns that read"] # [doc = " gets the number of bytes from the `Vec`'s length, not its capacity."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Reading zero bytes is almost certainly not the intended behavior."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " In theory, a very unusual read implementation could assign some semantic meaning"] # [doc = " to zero-byte reads. But it seems exceptionally unlikely that code intending to do"] # [doc = " a zero-byte read would allocate a `Vec` for it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " fn foo<F: io::Read>(mut f: F) {"] # [doc = "     let mut data = Vec::with_capacity(100);"] # [doc = "     f.read(&mut data).unwrap();"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " fn foo<F: io::Read>(mut f: F) {"] # [doc = "     let mut data = Vec::with_capacity(100);"] # [doc = "     data.resize(100, 0);"] # [doc = "     f.read(&mut data).unwrap();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub READ_ZERO_BYTE_VEC , nursery , "checks for reads into a zero-length `Vec`" }
};
}
