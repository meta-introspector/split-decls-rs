macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! LOCKED_BIT {
    () => {
        deps!();
        # [doc = " This bit is set in the `state` of a `RawMutex` when that mutex is locked by some thread."] const LOCKED_BIT : u8 = 0b01 ;
    };
}

LOCKED_BIT!()