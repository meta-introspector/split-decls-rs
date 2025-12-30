// Generated macro for impl_770 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_770 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_770"}
// Dependencies: {}
# [doc = " Collapses all unit items from a parallel iterator into one."] # [doc = ""] # [doc = " This is more useful when combined with higher-level abstractions, like"] # [doc = " collecting to a `Result<(), E>` where you only care about errors:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::*;"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let data = vec![1, 2, 3, 4, 5];"] # [doc = " let res: Result<()> = data.par_iter()"] # [doc = "     .map(|x| writeln!(stdout(), \"{}\", x))"] # [doc = "     .collect();"] # [doc = " assert!(res.is_ok());"] # [doc = " ```"] impl FromParallelIterator < () > for () { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = () > , { par_iter . into_par_iter () . drive_unindexed (NoopConsumer) } }
};
}
