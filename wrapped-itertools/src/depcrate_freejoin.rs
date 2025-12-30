// Generated macro for join (function)
macro_rules! Depcrate_freejoin {
() => {
// Module: crate::free
// Provides: {"join"}
// Dependencies: {}
# [doc = " Combine all iterator elements into one `String`, separated by `sep`."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::join`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::join;"] # [doc = ""] # [doc = " assert_eq!(join(&[1, 2, 3], \", \"), \"1, 2, 3\");"] # [doc = " ```"] # [cfg (feature = "use_alloc")] pub fn join < I > (iterable : I , sep : & str) -> String where I : IntoIterator , I :: Item : Display , { iterable . into_iter () . join (sep) }
};
}
