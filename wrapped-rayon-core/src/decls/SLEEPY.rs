macro_rules! deps {
    () => {
        Latch!();
    };
}

macro_rules! SLEEPY {
    () => {
        deps!();
        # [doc = " Latch is not set, owning thread is going to sleep on this latch"] # [doc = " (but has not yet fallen asleep)."] const SLEEPY : usize = 1 ;
    };
}

SLEEPY!()