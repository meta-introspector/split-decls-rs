macro_rules! deps {
    () => {
        Consumer!();
        Reducer!();
        ListStringConsumer!();
        Folder!();
        ListReducer!();
        ListStringFolder!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl Consumer < char > for ListStringConsumer { type Folder = ListStringFolder ; type Reducer = ListReducer ; type Result = LinkedList < String > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListStringFolder { string : String :: new () , } } fn full (& self) -> bool { false } }
    };
}

impl_445!();