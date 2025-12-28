macro_rules! deps {
    () => {
        Allocator!();
        Splice!();
        Vec!();
        IntoIter!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < T , A : Allocator > Vec < T , A > { # [doc = " Creates a splicing iterator that replaces the specified range in the vector"] # [doc = " with the given `replace_with` iterator and yields the removed items."] # [doc = " `replace_with` does not need to be the same length as `range`."] # [doc = ""] # [doc = " `range` is removed even if the iterator is not consumed until the end."] # [doc = ""] # [doc = " It is unspecified how many elements are removed from the vector"] # [doc = " if the `Splice` value is leaked."] # [doc = ""] # [doc = " The input iterator `replace_with` is only consumed when the `Splice` value is dropped."] # [doc = ""] # [doc = " This is optimal if:"] # [doc = ""] # [doc = " * The tail (elements in the vector after `range`) is empty,"] # [doc = " * or `replace_with` yields fewer or equal elements than `range`’s length"] # [doc = " * or the lower bound of its `size_hint()` is exact."] # [doc = ""] # [doc = " Otherwise, a temporary vector is allocated and the tail is moved twice."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the starting point is greater than the end point or if"] # [doc = " the end point is greater than the length of the vector."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " let mut v = vec![1, 2, 3, 4];"] # [doc = " let new = [7, 8, 9];"] # [doc = " let u: Vec<_> = v.splice(1..3, new).collect();"] # [doc = " assert_eq!(v, &[1, 7, 8, 9, 4]);"] # [doc = " assert_eq!(u, &[2, 3]);"] # [doc = " ```"] # [cfg (not (no_global_oom_handling))] # [inline (always)] pub fn splice < R , I > (& mut self , range : R , replace_with : I) -> Splice < '_ , I :: IntoIter , A > where R : RangeBounds < usize > , I : IntoIterator < Item = T > , { Splice { drain : self . drain (range) , replace_with : replace_with . into_iter () , } } }
    };
}

impl_170!();