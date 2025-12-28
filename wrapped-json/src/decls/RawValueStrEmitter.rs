macro_rules! deps {
    () => {
        Formatter!();
        Serializer!();
    };
}

macro_rules! RawValueStrEmitter {
    () => {
        deps!();
        # [cfg (feature = "raw_value")] struct RawValueStrEmitter < 'a , W : 'a + io :: Write , F : 'a + Formatter > (& 'a mut Serializer < W , F >) ;
    };
}

RawValueStrEmitter!();