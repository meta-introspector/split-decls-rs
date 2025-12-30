// Generated macro for UnwindOffset (trait)
macro_rules! Depcrate_read_cfiUnwindOffset {
() => {
// Module: crate::read::cfi
// Provides: {"UnwindOffset"}
// Dependencies: {}
# [doc = " An offset into an `UnwindSection`."] pub trait UnwindOffset < T = usize > : Copy + Debug + Eq + From < T > where T : ReaderOffset , { # [doc = " Convert an `UnwindOffset<T>` into a `T`."] fn into (self) -> T ; }
};
}
