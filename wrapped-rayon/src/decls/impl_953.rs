macro_rules! deps {
    () => {
        Reducer!();
        UnzipConsumer!();
        UnindexedConsumer!();
        UnzipOp!();
        UnzipReducer!();
    };
}

macro_rules! impl_953 {
    () => {
        deps!();
        impl < 'a , T , OP , CA , CB > UnindexedConsumer < T > for UnzipConsumer < 'a , OP , CA , CB > where OP : UnzipOp < T > , CA : UnindexedConsumer < OP :: Left > , CB : UnindexedConsumer < OP :: Right > , { fn split_off_left (& self) -> Self { UnzipConsumer { op : self . op , left : self . left . split_off_left () , right : self . right . split_off_left () , } } fn to_reducer (& self) -> Self :: Reducer { UnzipReducer { left : self . left . to_reducer () , right : self . right . to_reducer () , } } }
    };
}

impl_953!()