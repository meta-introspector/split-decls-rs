macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T , S1 , S2 > Sub < & IndexSet < T , S2 > > for & IndexSet < T , S1 > where T : Eq + Hash + Clone , S1 : BuildHasher + Default , S2 : BuildHasher , { type Output = IndexSet < T , S1 > ; # [doc = " Returns the set difference, cloned into a new set."] # [doc = ""] # [doc = " Values are collected in the same order that they appear in `self`."] fn sub (self , other : & IndexSet < T , S2 >) -> Self :: Output { self . difference (other) . cloned () . collect () } }
    };
}

impl_94!();