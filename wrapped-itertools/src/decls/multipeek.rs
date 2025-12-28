macro_rules! deps {
    () => {
        MultiPeek!();
    };
}

macro_rules! multipeek {
    () => {
        deps!();
        # [doc = " An iterator adaptor that allows the user to peek at multiple `.next()`"] # [doc = " values without advancing the base iterator."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::multipeek`]."] pub fn multipeek < I > (iterable : I) -> MultiPeek < I :: IntoIter > where I : IntoIterator , { MultiPeek { iter : iterable . into_iter () . fuse () , buf : VecDeque :: new () , index : 0 , } }
    };
}

multipeek!()