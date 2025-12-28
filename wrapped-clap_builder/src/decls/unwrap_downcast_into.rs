macro_rules! deps {
    () => {
        AnyValue!();
    };
}

macro_rules! unwrap_downcast_into {
    () => {
        deps!();
        # [track_caller] fn unwrap_downcast_into < T : Any + Clone + Send + Sync + 'static > (value : AnyValue) -> T { value . downcast_into () . expect (INTERNAL_ERROR_MSG) }
    };
}

unwrap_downcast_into!();