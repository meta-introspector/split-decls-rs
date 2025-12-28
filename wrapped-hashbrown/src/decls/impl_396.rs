macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl < T , S , A > PartialEq for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { fn eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| key | other . contains (key)) } }
    };
}

impl_396!();