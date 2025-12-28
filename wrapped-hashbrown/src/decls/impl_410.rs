macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl < T , S , A > BitAndAssign < & HashSet < T , S , A > > for HashSet < T , S , A > where T : Eq + Hash + Clone , S : BuildHasher , A : Allocator , { # [doc = " Modifies this set to contain the intersection of `self` and `rhs`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();"] # [doc = " let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();"] # [doc = ""] # [doc = " a &= &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [2, 3];"] # [doc = " for x in &a {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn bitand_assign (& mut self , rhs : & HashSet < T , S , A >) { self . retain (| item | rhs . contains (item)) ; } }
    };
}

impl_410!();