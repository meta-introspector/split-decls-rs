macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T , S > BitAnd < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitand (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . intersection (rhs) . cloned () . collect () } }
    };
}

impl_146!();