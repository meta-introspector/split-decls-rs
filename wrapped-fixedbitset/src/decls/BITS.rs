macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! BITS {
    () => {
        deps!();
        pub (crate) const BITS : usize = core :: mem :: size_of :: < Block > () * 8 ;
    };
}

BITS!()