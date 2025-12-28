macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
        IntoIter!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a , T , A : Allocator > IntoIterator for & 'a Vec < T , A > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_167!()