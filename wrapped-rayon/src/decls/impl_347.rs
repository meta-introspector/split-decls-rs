macro_rules! deps {
    () => {
        Consumer!();
        CollectConsumer!();
        Folder!();
        CollectReducer!();
        Reducer!();
        CollectResult!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < 'c , T : Send + 'c > Consumer < T > for CollectConsumer < 'c , T > { type Folder = CollectResult < 'c , T > ; type Reducer = CollectReducer ; type Result = CollectResult < 'c , T > ; fn split_at (self , index : usize) -> (Self , Self , CollectReducer) { let CollectConsumer { start , len , .. } = self ; unsafe { assert ! (index <= len) ; (CollectConsumer :: new (start . 0 , index) , CollectConsumer :: new (start . 0 . add (index) , len - index) , CollectReducer ,) } } fn into_folder (self) -> Self :: Folder { CollectResult { start : self . start , total_len : self . len , initialized_len : 0 , invariant_lifetime : PhantomData , } } fn full (& self) -> bool { false } }
    };
}

impl_347!();