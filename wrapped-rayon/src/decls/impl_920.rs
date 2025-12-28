macro_rules! deps {
    () => {
        Reducer!();
        TryReduceConsumer!();
    };
}

macro_rules! impl_920 {
    () => {
        deps!();
        impl < 'r , R , ID , T > Reducer < T > for TryReduceConsumer < 'r , R , ID > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try , { fn reduce (self , left : T , right : T) -> T { match (left . branch () , right . branch ()) { (Continue (left) , Continue (right)) => (self . reduce_op) (left , right) , (Break (r) , _) | (_ , Break (r)) => T :: from_residual (r) , } } }
    };
}

impl_920!()