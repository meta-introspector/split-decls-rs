macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        CollectConsumer!();
        CollectReducer!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        # [doc = " Pretend to be unindexed for `special_collect_into_vec`,"] # [doc = " but we should never actually get used that way..."] impl < 'c , T : Send + 'c > UnindexedConsumer < T > for CollectConsumer < 'c , T > { fn split_off_left (& self) -> Self { unreachable ! ("CollectConsumer must be indexed!") } fn to_reducer (& self) -> Self :: Reducer { CollectReducer } }
    };
}

impl_349!();