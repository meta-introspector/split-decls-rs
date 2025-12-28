macro_rules! deps {
    () => {
        Consumer!();
    };
}

macro_rules! UnzipConsumer {
    () => {
        deps!();
        # [doc = " `Consumer` that unzips into two other `Consumer`s"] struct UnzipConsumer < 'a , OP , CA , CB > { op : & 'a OP , left : CA , right : CB , }
    };
}

UnzipConsumer!()