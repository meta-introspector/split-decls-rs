macro_rules! deps {
    () => {
        Consumer!();
        Folder!();
        ReduceConsumer!();
        Reducer!();
        ReduceFolder!();
    };
}

macro_rules! impl_789 {
    () => {
        deps!();
        impl < 'r , R , ID , T > Consumer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , ID : Fn () -> T + Sync , T : Send , { type Folder = ReduceFolder < 'r , R , T > ; type Reducer = Self ; type Result = T ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (self , self , self) } fn into_folder (self) -> Self :: Folder { ReduceFolder { reduce_op : self . reduce_op , item : (self . identity) () , } } fn full (& self) -> bool { false } }
    };
}

impl_789!()