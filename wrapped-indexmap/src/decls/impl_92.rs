macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T , S1 , S2 > BitOr < & IndexSet < T , S2 > > for & IndexSet < T , S1 > where T : Eq + Hash + Clone , S1 : BuildHasher + Default , S2 : BuildHasher , { type Output = IndexSet < T , S1 > ; # [doc = " Returns the set union, cloned into a new set."] # [doc = ""] # [doc = " Values from `self` are collected in their original order, followed by"] # [doc = " values that are unique to `other` in their original order."] fn bitor (self , other : & IndexSet < T , S2 >) -> Self :: Output { self . union (other) . cloned () . collect () } }
    };
}

impl_92!()