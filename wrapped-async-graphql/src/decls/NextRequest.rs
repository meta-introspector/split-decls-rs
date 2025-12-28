macro_rules! deps {
    () => {
        Extension!();
        RequestFut!();
    };
}

macro_rules! NextRequest {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for request."] pub struct NextRequest < 'a > { chain : & 'a [Arc < dyn Extension >] , request_fut : RequestFut < 'a > , }
    };
}

NextRequest!();