macro_rules! deps {
    () => {
        HeadTail!();
        KMergeBy!();
        KMergePredicate!();
    };
}

macro_rules! kmerge_by {
    () => {
        deps!();
        # [doc = " Create an iterator that merges elements of the contained iterators."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::kmerge_by`](crate::Itertools::kmerge_by)."] pub fn kmerge_by < I , F > (iterable : I , mut less_than : F ,) -> KMergeBy < < I :: Item as IntoIterator > :: IntoIter , F > where I : IntoIterator , I :: Item : IntoIterator , F : KMergePredicate < < < I as IntoIterator > :: Item as IntoIterator > :: Item > , { let iter = iterable . into_iter () ; let (lower , _) = iter . size_hint () ; let mut heap : Vec < _ > = Vec :: with_capacity (lower) ; heap . extend (iter . filter_map (| it | HeadTail :: new (it . into_iter ()))) ; heapify (& mut heap , | a , b | less_than . kmerge_pred (& a . head , & b . head)) ; KMergeBy { heap , less_than } }
    };
}

kmerge_by!()