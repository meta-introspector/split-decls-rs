macro_rules! deps {
    () => {
        ListFolder!();
        Folder!();
        Consumer!();
        ListConsumer!();
        ListReducer!();
        Reducer!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < T : Send > Consumer < T > for ListConsumer { type Folder = ListFolder < T > ; type Reducer = ListReducer ; type Result = LinkedList < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListFolder { list : LinkedList :: new () , } } fn full (& self) -> bool { false } }
    };
}

impl_434!()