macro_rules! deps {
    () => {
        Eager!();
        BlockBuffer!();
    };
}

macro_rules! EagerBuffer {
    () => {
        deps!();
        # [doc = " Eager block buffer."] pub type EagerBuffer < B > = BlockBuffer < B , Eager > ;
    };
}

EagerBuffer!()