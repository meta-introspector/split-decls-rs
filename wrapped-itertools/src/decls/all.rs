macro_rules! all {
    () => {
        # [doc = " Test whether the predicate holds for all elements in the iterable."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::all`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::all;"] # [doc = ""] # [doc = " assert!(all(&[1, 2, 3], |elt| *elt > 0));"] # [doc = " ```"] pub fn all < I , F > (iterable : I , f : F) -> bool where I : IntoIterator , F : FnMut (I :: Item) -> bool , { iterable . into_iter () . all (f) }
    };
}

all!()