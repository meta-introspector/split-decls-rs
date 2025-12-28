macro_rules! deps {
    () => {
        PunycodeCaller!();
        PunycodeCodeUnit!();
    };
}

macro_rules! Decode {
    () => {
        deps!();
        pub (crate) struct Decode < 'a , T , C > where T : PunycodeCodeUnit + Copy , C : PunycodeCaller , { base : core :: slice :: Iter < 'a , T > , pub (crate) insertions : & 'a [(usize , char)] , inserted : usize , position : usize , len : usize , phantom : PhantomData < C > , }
    };
}

Decode!();