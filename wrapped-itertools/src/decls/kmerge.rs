macro_rules! deps {
    () => {
        KMergeByLt!();
        KMerge!();
    };
}

macro_rules! kmerge {
    () => {
        deps!();
        # [doc = " Create an iterator that merges elements of the contained iterators using"] # [doc = " the ordering function."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::kmerge`](crate::Itertools::kmerge)."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::kmerge;"] # [doc = ""] # [doc = " for elt in kmerge(vec![vec![0, 2, 4], vec![1, 3, 5], vec![6, 7]]) {"] # [doc = "     /* loop body */"] # [doc = "     # let _ = elt;"] # [doc = " }"] # [doc = " ```"] pub fn kmerge < I > (iterable : I) -> KMerge < < I :: Item as IntoIterator > :: IntoIter > where I : IntoIterator , I :: Item : IntoIterator , < < I as IntoIterator > :: Item as IntoIterator > :: Item : PartialOrd , { kmerge_by (iterable , KMergeByLt) }
    };
}

kmerge!()