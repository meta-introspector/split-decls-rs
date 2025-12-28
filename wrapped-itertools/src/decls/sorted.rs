macro_rules! deps {
    () => {
        VecIntoIter!();
    };
}

macro_rules! sorted {
    () => {
        deps!();
        # [doc = " Sort all iterator elements into a new iterator in ascending order."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::sorted`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::assert_equal;"] # [doc = " use itertools::sorted;"] # [doc = ""] # [doc = " assert_equal(sorted(\"rust\".chars()), \"rstu\".chars());"] # [doc = " ```"] # [cfg (feature = "use_alloc")] pub fn sorted < I > (iterable : I) -> VecIntoIter < I :: Item > where I : IntoIterator , I :: Item : Ord , { iterable . into_iter () . sorted () }
    };
}

sorted!();