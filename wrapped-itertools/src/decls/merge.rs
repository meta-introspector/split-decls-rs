macro_rules! deps {
    () => {
        MergeLte!();
        Merge!();
    };
}

macro_rules! merge {
    () => {
        deps!();
        # [doc = " Create an iterator that merges elements in `i` and `j`."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::merge`](crate::Itertools::merge)."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::merge;"] # [doc = ""] # [doc = " for elt in merge(&[1, 2, 3], &[2, 3, 4]) {"] # [doc = "     /* loop body */"] # [doc = "     # let _ = elt;"] # [doc = " }"] # [doc = " ```"] pub fn merge < I , J > (i : I , j : J ,) -> Merge < < I as IntoIterator > :: IntoIter , < J as IntoIterator > :: IntoIter > where I : IntoIterator , J : IntoIterator < Item = I :: Item > , I :: Item : PartialOrd , { merge_by_new (i , j , MergeLte) }
    };
}

merge!();