// Generated macro for IntersperseWith (struct)
macro_rules! Depcrate_intersperseIntersperseWith {
() => {
// Module: crate::intersperse
// Provides: {"IntersperseWith"}
// Dependencies: {}
# [doc = " An iterator adaptor to insert a particular value created by a function"] # [doc = " between each element of the adapted iterator."] # [doc = ""] # [doc = " Iterator element type is `I::Item`"] # [doc = ""] # [doc = " This iterator is *fused*."] # [doc = ""] # [doc = " See [`.intersperse_with()`](crate::Itertools::intersperse_with) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct IntersperseWith < I , ElemF > where I : Iterator , { element : ElemF , iter : Fuse < I > , # [doc = " `peek` is None while no item have been taken out of `iter` (at definition)."] # [doc = " Then `peek` will alternatively be `Some(None)` and `Some(Some(item))`,"] # [doc = " where `None` indicates it's time to generate from `element` (unless `iter` is empty)."] peek : Option < Option < I :: Item > > , }
};
}
