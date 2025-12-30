// Generated macro for impl_359 (impl)
macro_rules! Depcrateimpl_359 {
() => {
// Module: crate
// Provides: {"impl_359"}
// Dependencies: {}
impl Compression { # [doc = " Creates a new description of the compression level with an explicitly"] # [doc = " specified integer."] # [doc = ""] # [doc = " The integer here is typically on a scale of 0-9 where 0 means \"no"] # [doc = " compression\" and 9 means \"take as long as you'd like\"."] pub const fn new (level : u32) -> Compression { Compression (level) } # [doc = " No compression is to be performed, this may actually inflate data"] # [doc = " slightly when encoding."] pub const fn none () -> Compression { Compression (0) } # [doc = " Optimize for the best speed of encoding."] pub const fn fast () -> Compression { Compression (1) } # [doc = " Optimize for the size of data being encoded."] pub const fn best () -> Compression { Compression (9) } # [doc = " Returns an integer representing the compression level, typically on a"] # [doc = " scale of 0-9. See [`new`](Self::new) for details about compression levels."] pub fn level (& self) -> u32 { self . 0 } }
};
}
