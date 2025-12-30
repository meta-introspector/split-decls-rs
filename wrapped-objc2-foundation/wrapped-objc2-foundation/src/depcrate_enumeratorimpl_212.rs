// Generated macro for impl_212 (impl)
macro_rules! Depcrate_enumeratorimpl_212 {
() => {
// Module: crate::enumerator
// Provides: {"impl_212"}
// Dependencies: {}
impl < ObjectType : Message > NSEnumerator < ObjectType > { # [doc = " Iterate over the enumerator's elements."] # [inline] pub fn iter (& self) -> Iter < '_ , ObjectType > { Iter (iter :: Iter :: new (self)) } # [doc = " Iterate over the enumerator without retaining the elements."] # [doc = ""] # [doc = " Consider using the [`iter`](Self::iter) method instead, unless you're"] # [doc = " seeing performance issues from the retaining."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The enumerator and the underlying collection must not be mutated while"] # [doc = " the iterator is alive."] # [inline] pub unsafe fn iter_unchecked (& self) -> IterUnchecked < '_ , ObjectType > { IterUnchecked (iter :: IterUnchecked :: new (self)) } }
};
}
