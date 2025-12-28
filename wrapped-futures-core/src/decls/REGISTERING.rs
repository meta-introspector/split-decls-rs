macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! REGISTERING {
    () => {
        deps!();
        # [doc = " A new waker value is being registered with the `AtomicWaker` cell."] const REGISTERING : usize = 0b01 ;
    };
}

REGISTERING!();