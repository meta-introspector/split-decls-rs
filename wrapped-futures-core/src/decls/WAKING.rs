macro_rules! deps {
    () => {
        AtomicWaker!();
    };
}

macro_rules! WAKING {
    () => {
        deps!();
        # [doc = " The waker currently registered with the `AtomicWaker` cell is being woken."] const WAKING : usize = 0b10 ;
    };
}

WAKING!();