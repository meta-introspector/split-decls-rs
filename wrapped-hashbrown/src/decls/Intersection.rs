macro_rules! deps {
    () => {
        HashSet!();
        Iter!();
    };
}

macro_rules! Intersection {
    () => {
        deps!();
        # [doc = " A lazy iterator producing elements in the intersection of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`intersection`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`intersection`]: struct.HashSet.html#method.intersection"] pub struct Intersection < 'a , T , S , A : Allocator = Global > { iter : Iter < 'a , T > , other : & 'a HashSet < T , S , A > , }
    };
}

Intersection!();