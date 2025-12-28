macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < T , S > Sub < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn sub (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . difference (rhs) . cloned () . collect () } }
    };
}

impl_148!();