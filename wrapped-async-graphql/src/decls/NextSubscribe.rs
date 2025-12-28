macro_rules! deps {
    () => {
        Extension!();
    };
}

macro_rules! NextSubscribe {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for subscribe."] pub struct NextSubscribe < 'a > { chain : & 'a [Arc < dyn Extension >] , }
    };
}

NextSubscribe!()