macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! ParUnion {
    () => {
        deps!();
        # [doc = " Parallel iterator over shared references to elements in the union of sets."] # [doc = ""] # [doc = " This iterator is created by the [`par_union`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`par_union`]: /hashbrown/struct.HashSet.html#method.par_union"] # [doc = " [`HashSet`]: /hashbrown/struct.HashSet.html"] pub struct ParUnion < 'a , T , S , A : Allocator = Global > { a : & 'a HashSet < T , S , A > , b : & 'a HashSet < T , S , A > , }
    };
}

ParUnion!();