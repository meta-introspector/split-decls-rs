// Generated macro for Comparable (trait)
macro_rules! Depcrate_equivalentComparable {
() => {
// Module: crate::equivalent
// Provides: {"Comparable"}
// Dependencies: {}
# [doc = " Key ordering trait."] # [doc = ""] # [doc = " This trait allows ordered map lookup to be customized. It has one blanket"] # [doc = " implementation that uses the regular solution with `Borrow` and `Ord`, just"] # [doc = " like `BTreeMap` does, so that you can pass `&str` to lookup into a map with"] # [doc = " `String` keys and so on."] pub trait Comparable < Q : ? Sized > : Equivalent < Q > { # [doc = " Compare self to `key` and return their ordering."] fn compare (& self , key : & Q) -> Ordering ; }
};
}
