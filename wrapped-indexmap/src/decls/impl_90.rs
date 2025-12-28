macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < T , S > IndexSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [doc = " Returns `true` if `self` has no elements in common with `other`."] pub fn is_disjoint < S2 > (& self , other : & IndexSet < T , S2 >) -> bool where S2 : BuildHasher , { if self . len () <= other . len () { self . iter () . all (move | value | ! other . contains (value)) } else { other . iter () . all (move | value | ! self . contains (value)) } } # [doc = " Returns `true` if all elements of `self` are contained in `other`."] pub fn is_subset < S2 > (& self , other : & IndexSet < T , S2 >) -> bool where S2 : BuildHasher , { self . len () <= other . len () && self . iter () . all (move | value | other . contains (value)) } # [doc = " Returns `true` if all elements of `other` are contained in `self`."] pub fn is_superset < S2 > (& self , other : & IndexSet < T , S2 >) -> bool where S2 : BuildHasher , { other . is_subset (self) } }
    };
}

impl_90!()