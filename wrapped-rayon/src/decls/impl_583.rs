macro_rules! deps {
    () => {
        Consumer!();
        Folder!();
        NoopReducer!();
        ForEachConsumer!();
        Reducer!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < 'f , F , T > Consumer < T > for ForEachConsumer < 'f , F > where F : Fn (T) + Sync , { type Folder = ForEachConsumer < 'f , F > ; type Reducer = NoopReducer ; type Result = () ; fn split_at (self , _index : usize) -> (Self , Self , NoopReducer) { (self . split_off_left () , self , NoopReducer) } fn into_folder (self) -> Self { self } fn full (& self) -> bool { false } }
    };
}

impl_583!()