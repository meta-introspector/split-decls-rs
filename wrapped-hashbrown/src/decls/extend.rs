macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! extend {
    () => {
        deps!();
        fn extend < T , S , I , A > (set : & mut HashSet < T , S , A > , par_iter : I) where T : Eq + Hash , S : BuildHasher , A : Allocator , I : IntoParallelIterator , HashSet < T , S , A > : Extend < I :: Item > , { let (list , len) = super :: helpers :: collect (par_iter) ; let reserve = if set . is_empty () { len } else { (len + 1) / 2 } ; set . reserve (reserve) ; for vec in list { set . extend (vec) ; } }
    };
}

extend!();