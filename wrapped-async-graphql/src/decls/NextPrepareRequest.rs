macro_rules! deps {
    () => {
        Extension!();
    };
}

macro_rules! NextPrepareRequest {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for subscribe."] pub struct NextPrepareRequest < 'a > { chain : & 'a [Arc < dyn Extension >] , }
    };
}

NextPrepareRequest!();