macro_rules! deps {
    () => {
        MergeJoinBy!();
        MergeBy!();
        MergeFuncLR!();
    };
}

macro_rules! merge_join_by {
    () => {
        deps!();
        # [doc = " Return an iterator adaptor that merge-joins items from the two base iterators in ascending order."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::merge_join_by`]."] pub fn merge_join_by < I , J , F , T > (left : I , right : J , cmp_fn : F ,) -> MergeJoinBy < I :: IntoIter , J :: IntoIter , F > where I : IntoIterator , J : IntoIterator , F : FnMut (& I :: Item , & J :: Item) -> T , { MergeBy { left : put_back (left . into_iter () . fuse ()) , right : put_back (right . into_iter () . fuse ()) , cmp_fn : MergeFuncLR (cmp_fn , PhantomData) , } }
    };
}

merge_join_by!()