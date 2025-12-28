macro_rules! deps {
    () => {
        TryReduceWithFolder!();
        Folder!();
        TryReduceWithConsumer!();
        Reducer!();
        Consumer!();
    };
}

macro_rules! impl_928 {
    () => {
        deps!();
        impl < 'r , R , T > Consumer < T > for TryReduceWithConsumer < 'r , R > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try + Send , { type Folder = TryReduceWithFolder < 'r , R , T > ; type Reducer = Self ; type Result = Option < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (self , self , self) } fn into_folder (self) -> Self :: Folder { TryReduceWithFolder { reduce_op : self . reduce_op , opt_control : None , full : self . full , } } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) } }
    };
}

impl_928!();