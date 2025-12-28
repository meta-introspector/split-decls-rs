macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! UnusedMut {
    () => {
        deps!();
        # [derive (Debug)] pub struct UnusedMut { pub local : Local , }
    };
}

UnusedMut!()