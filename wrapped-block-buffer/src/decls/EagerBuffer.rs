macro_rules! deps {
    () => {
        BlockBuffer!();
        Eager!();
    };
}

macro_rules! EagerBuffer {
    () => {
        deps!();
        # [doc = " Eager block buffer."] pub type EagerBuffer < B > = BlockBuffer < B , Eager > ;
    };
}

EagerBuffer!();