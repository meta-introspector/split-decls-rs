macro_rules! deps {
    () => {
        WithCount!();
        CountItem!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > CountItem < T > for WithCount { type CItem = (usize , T) ; # [inline (always)] fn new (t : T) -> (usize , T) { (1 , t) } }
    };
}

impl_21!()