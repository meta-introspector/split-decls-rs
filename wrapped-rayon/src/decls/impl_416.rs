macro_rules! deps {
    () => {
        Folder!();
        ListVecFolder!();
        Reducer!();
        Consumer!();
        ListReducer!();
        ListVecConsumer!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl < T : Send > Consumer < T > for ListVecConsumer { type Folder = ListVecFolder < T > ; type Reducer = ListReducer ; type Result = LinkedList < Vec < T > > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListVecFolder { vec : Vec :: new () } } fn full (& self) -> bool { false } }
    };
}

impl_416!();