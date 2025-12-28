macro_rules! deps {
    () => {
        BlockBuffer!();
        Lazy!();
    };
}

macro_rules! LazyBuffer {
    () => {
        deps!();
        # [doc = " Lazy block buffer."] pub type LazyBuffer < B > = BlockBuffer < B , Lazy > ;
    };
}

LazyBuffer!();