macro_rules! deps {
    () => {
        Reducer!();
        NoopReducer!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl Reducer < () > for NoopReducer { fn reduce (self , _left : () , _right : ()) { } }
    };
}

impl_729!();