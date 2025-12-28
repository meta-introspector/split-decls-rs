macro_rules! deps {
    () => {
        Iter!();
        LinkedHashSet!();
        IntoIter!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < 'a , T , S > IntoIterator for & 'a LinkedHashSet < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; # [inline] fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
    };
}

impl_156!()