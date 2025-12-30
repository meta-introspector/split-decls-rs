// Generated macro for Equivalent (trait)
macro_rules! Depcrate_equivalentEquivalent {
() => {
// Module: crate::equivalent
// Provides: {"Equivalent"}
// Dependencies: {}
# [doc = " Key equivalence trait."] # [doc = ""] # [doc = " This trait allows hash table lookup to be customized. It has one blanket"] # [doc = " implementation that uses the regular solution with `Borrow` and `Eq`, just"] # [doc = " like `HashMap` does, so that you can pass `&str` to lookup into a map with"] # [doc = " `String` keys and so on."] # [doc = ""] # [doc = " # Contract"] # [doc = ""] # [doc = " The implementor **must** hash like `Q`, if it is hashable."] pub trait Equivalent < Q : ? Sized > { # [doc = " Compare self to `key` and return `true` if they are equal."] fn equivalent (& self , key : & Q) -> bool ; }
};
}
