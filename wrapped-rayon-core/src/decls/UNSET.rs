macro_rules! deps {
    () => {
        Latch!();
    };
}

macro_rules! UNSET {
    () => {
        deps!();
        # [doc = " Latch is not set, owning thread is awake"] const UNSET : usize = 0 ;
    };
}

UNSET!();