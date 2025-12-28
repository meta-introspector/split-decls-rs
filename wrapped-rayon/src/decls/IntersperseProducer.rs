macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! IntersperseProducer {
    () => {
        deps!();
        struct IntersperseProducer < P > where P : Producer , { base : P , item : P :: Item , len : usize , clone_first : bool , }
    };
}

IntersperseProducer!();