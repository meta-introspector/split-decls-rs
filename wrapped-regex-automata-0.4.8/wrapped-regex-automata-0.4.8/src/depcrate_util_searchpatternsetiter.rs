// Generated macro for PatternSetIter (struct)
macro_rules! Depcrate_util_searchPatternSetIter {
() => {
// Module: crate::util::search
// Provides: {"PatternSetIter"}
// Dependencies: {}
# [doc = " An iterator over all pattern identifiers in a [`PatternSet`]."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the pattern set being"] # [doc = " iterated over."] # [doc = ""] # [doc = " This iterator is created by the [`PatternSet::iter`] method."] # [cfg (feature = "alloc")] # [derive (Clone , Debug)] pub struct PatternSetIter < 'a > { it : core :: iter :: Enumerate < core :: slice :: Iter < 'a , bool > > , }
};
}
