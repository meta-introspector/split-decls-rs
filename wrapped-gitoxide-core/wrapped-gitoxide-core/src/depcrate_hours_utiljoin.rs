// Generated macro for join (function)
macro_rules! Depcrate_hours_utiljoin {
() => {
// Module: crate::hours::util
// Provides: {"join"}
// Dependencies: {}
# [doc = " Combine all iterator elements into one String, separated by `sep`."] # [doc = ""] # [doc = " Use the `Display` implementation of each element."] # [doc = ""] # [doc = " extracted from"] # [doc = " https://github.com/rust-itertools/itertools/blob/762643f1be2217140a972745cf4d6ed69435f722/src/lib.rs#L2295-L2324"] fn join < I > (mut iter : I , sep : & str) -> String where I : Iterator , < I as Iterator > :: Item : std :: fmt :: Display , { use :: std :: fmt :: Write ; match iter . next () { None => String :: new () , Some (first_elt) => { let (lower , _) = iter . size_hint () ; let mut result = String :: with_capacity (sep . len () * lower) ; write ! (& mut result , "{first_elt}") . expect ("enough memory") ; iter . for_each (| elt | { result . push_str (sep) ; write ! (& mut result , "{elt}") . expect ("enough memory") ; }) ; result } } }
};
}
