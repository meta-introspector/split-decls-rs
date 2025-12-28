macro_rules! deps {
    () => {
        HashSet!();
        DefaultHashBuilder!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        # [cfg (feature = "default-hasher")] impl < T , A , const N : usize > From < [T ; N] > for HashSet < T , DefaultHashBuilder , A > where T : Eq + Hash , A : Default + Allocator , { # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let set1 = HashSet::from([1, 2, 3, 4]);"] # [doc = " let set2: HashSet<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(set1, set2);"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { arr . into_iter () . collect () } }
    };
}

impl_401!()