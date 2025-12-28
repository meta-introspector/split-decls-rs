macro_rules! deps {
    () => {
        Iter!();
        Repository!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'packed , 'repo > Iter < 'packed , 'repo > { fn new (repo : & 'repo crate :: Repository , platform : gix_ref :: file :: iter :: LooseThenPacked < 'packed , 'repo >) -> Self { Iter { inner : platform , peel_with_packed : None , peel : false , repo , } } }
    };
}

impl_256!();