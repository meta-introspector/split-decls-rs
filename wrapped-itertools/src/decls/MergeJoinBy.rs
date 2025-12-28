macro_rules! deps {
    () => {
        FuncLR!();
        MergeBy!();
        MergeFuncLR!();
    };
}

macro_rules! MergeJoinBy {
    () => {
        deps!();
        # [doc = " An iterator adaptor that merge-joins items from the two base iterators in ascending order."] # [doc = ""] # [doc = " See [`.merge_join_by()`](crate::Itertools::merge_join_by) for more information."] pub type MergeJoinBy < I , J , F > = MergeBy < I , J , MergeFuncLR < F , < F as FuncLR < < I as Iterator > :: Item , < J as Iterator > :: Item > > :: T > > ;
    };
}

MergeJoinBy!()