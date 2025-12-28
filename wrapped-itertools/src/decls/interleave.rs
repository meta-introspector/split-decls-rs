macro_rules! deps {
    () => {
        Interleave!();
    };
}

macro_rules! interleave {
    () => {
        deps!();
        # [doc = " Create an iterator that interleaves elements in `i` and `j`."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::interleave`](crate::Itertools::interleave)."] pub fn interleave < I , J > (i : I , j : J ,) -> Interleave < < I as IntoIterator > :: IntoIter , < J as IntoIterator > :: IntoIter > where I : IntoIterator , J : IntoIterator < Item = I :: Item > , { Interleave { i : i . into_iter () . fuse () , j : j . into_iter () . fuse () , next_coming_from_j : false , } }
    };
}

interleave!();