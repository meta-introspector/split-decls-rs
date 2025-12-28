macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < T , S > BitXor < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitxor (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . symmetric_difference (rhs) . cloned () . collect () } }
    };
}

impl_147!()