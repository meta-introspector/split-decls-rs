macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T , const N : usize > From < [T ; N] > for IndexSet < T , RandomState > where T : Eq + Hash , { # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexSet;"] # [doc = ""] # [doc = " let set1 = IndexSet::from([1, 2, 3, 4]);"] # [doc = " let set2: IndexSet<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(set1, set2);"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { Self :: from_iter (arr) } }
    };
}

impl_84!();