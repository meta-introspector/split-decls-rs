macro_rules! deps {
    () => {
        PatternSet!();
    };
}

macro_rules! PatternSetIter {
    () => {
        deps!();
        # [doc = " An iterator over all pattern identifiers in a [`PatternSet`]."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the pattern set being"] # [doc = " iterated over."] # [doc = ""] # [doc = " This iterator is created by the [`PatternSet::iter`] method."] # [cfg (feature = "alloc")] # [derive (Clone , Debug)] pub struct PatternSetIter < 'a > { it : core :: iter :: Enumerate < core :: slice :: Iter < 'a , bool > > , }
    };
}

PatternSetIter!();