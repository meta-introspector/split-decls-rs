macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T , S > BitOr < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitor (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . union (rhs) . cloned () . collect () } }
    };
}

impl_145!();