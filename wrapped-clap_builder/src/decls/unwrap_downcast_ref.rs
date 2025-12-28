macro_rules! deps {
    () => {
        AnyValue!();
    };
}

macro_rules! unwrap_downcast_ref {
    () => {
        deps!();
        # [track_caller] fn unwrap_downcast_ref < T : Any + Clone + Send + Sync + 'static > (value : & AnyValue) -> & T { value . downcast_ref () . expect (INTERNAL_ERROR_MSG) }
    };
}

unwrap_downcast_ref!()