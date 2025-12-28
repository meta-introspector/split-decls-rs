macro_rules! deps {
    () => {
        Folder!();
        Reducer!();
        NoopConsumer!();
        NoopReducer!();
        Consumer!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl < T > Consumer < T > for NoopConsumer { type Folder = NoopConsumer ; type Reducer = NoopReducer ; type Result = () ; fn split_at (self , _index : usize) -> (Self , Self , NoopReducer) { (NoopConsumer , NoopConsumer , NoopReducer) } fn into_folder (self) -> Self { self } fn full (& self) -> bool { false } }
    };
}

impl_725!();