// Generated macro for macro_7185 (macro)
macro_rules! Depcrate_methodsmacro_7185 {
() => {
// Module: crate::methods
// Provides: {"macro_7185"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for naive byte counts"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The [`bytecount`](https://crates.io/crates/bytecount)"] # [doc = " crate has methods to count your bytes faster, especially for large slices."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If you have predominantly small slices, the"] # [doc = " `bytecount::count(..)` method may actually be slower. However, if you can"] # [doc = " ensure that less than 2³²-1 matches arise, the `naive_count_32(..)` can be"] # [doc = " faster in those cases."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let vec = vec![1_u8];"] # [doc = " let count = vec.iter().filter(|x| **x == 0u8).count();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " # let vec = vec![1_u8];"] # [doc = " let count = bytecount::count(&vec, 0u8);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NAIVE_BYTECOUNT , pedantic , "use of naive `<slice>.filter(|&x| x == y).count()` to count byte values" }
};
}
