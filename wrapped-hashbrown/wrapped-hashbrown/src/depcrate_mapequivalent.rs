// Generated macro for equivalent (function)
macro_rules! Depcrate_mapequivalent {
() => {
// Module: crate::map
// Provides: {"equivalent"}
// Dependencies: {}
# [doc = " Ensures that a single closure type across uses of this which, in turn prevents multiple"] # [doc = " instances of any functions like `RawTable::reserve` from being generated"] # [cfg_attr (feature = "inline-more" , inline)] # [allow (dead_code)] pub (crate) fn equivalent < Q , K > (k : & Q) -> impl Fn (& K) -> bool + '_ where Q : Equivalent < K > + ? Sized , { move | x | k . equivalent (x) }
};
}
