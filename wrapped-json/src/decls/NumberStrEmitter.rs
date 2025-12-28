macro_rules! deps {
    () => {
        Serializer!();
        Formatter!();
    };
}

macro_rules! NumberStrEmitter {
    () => {
        deps!();
        # [cfg (feature = "arbitrary_precision")] struct NumberStrEmitter < 'a , W : 'a + io :: Write , F : 'a + Formatter > (& 'a mut Serializer < W , F >) ;
    };
}

NumberStrEmitter!();