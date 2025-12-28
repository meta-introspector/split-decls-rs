macro_rules! deps {
    () => {
        FindReducer!();
        FindFolder!();
        FindConsumer!();
        Folder!();
        Reducer!();
        Consumer!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < 'p , T , P : 'p > Consumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { type Folder = FindFolder < 'p , T , P > ; type Reducer = FindReducer ; type Result = Option < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (self . split_off_left () , self , FindReducer) } fn into_folder (self) -> Self :: Folder { FindFolder { find_op : self . find_op , found : self . found , item : None , } } fn full (& self) -> bool { self . found . load (Ordering :: Relaxed) } }
    };
}

impl_483!();