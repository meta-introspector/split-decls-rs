macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! BYTES {
    () => {
        deps!();
        # [cfg (feature = "serde")] pub (crate) const BYTES : usize = core :: mem :: size_of :: < Block > () ;
    };
}

BYTES!()