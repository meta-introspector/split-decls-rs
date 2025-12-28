macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < T , S , A > BitXorAssign < & HashSet < T , S , A > > for HashSet < T , S , A > where T : Eq + Hash + Clone , S : BuildHasher , A : Allocator , { # [doc = " Modifies this set to contain the symmetric difference of `self` and `rhs`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();"] # [doc = " let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();"] # [doc = ""] # [doc = " a ^= &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [1, 2, 4, 5];"] # [doc = " for x in &a {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn bitxor_assign (& mut self , rhs : & HashSet < T , S , A >) { for item in rhs { let hash = make_hash (& self . map . hash_builder , item) ; match self . map . find_or_find_insert_index (hash , item) { Ok (bucket) => unsafe { self . map . table . remove (bucket) ; } , Err (index) => unsafe { self . map . table . insert_at_index (hash , index , (item . clone () , ())) ; } , } } } }
    };
}

impl_411!()