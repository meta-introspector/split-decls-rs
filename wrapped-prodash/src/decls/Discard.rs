macro_rules! deps {
    () => {
        NestedProgress!();
    };
}

macro_rules! Discard {
    () => {
        deps!();
        # [doc = " An implementation of [`NestedProgress`] which discards all calls."] pub struct Discard ;
    };
}

Discard!()