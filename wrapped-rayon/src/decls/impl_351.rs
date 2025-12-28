macro_rules! deps {
    () => {
        CollectResult!();
        CollectReducer!();
        Reducer!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < 'c , T > Reducer < CollectResult < 'c , T > > for CollectReducer { fn reduce (self , mut left : CollectResult < 'c , T > , right : CollectResult < 'c , T > ,) -> CollectResult < 'c , T > { unsafe { let left_end = left . start . 0 . add (left . initialized_len) ; if left_end == right . start . 0 { left . total_len += right . total_len ; left . initialized_len += right . release_ownership () ; } left } } }
    };
}

impl_351!();