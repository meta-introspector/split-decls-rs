macro_rules! deps {
    () => {
        ReduceConsumer!();
        Reducer!();
    };
}

macro_rules! impl_791 {
    () => {
        deps!();
        impl < 'r , R , ID , T > Reducer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , { fn reduce (self , left : T , right : T) -> T { (self . reduce_op) (left , right) } }
    };
}

impl_791!()