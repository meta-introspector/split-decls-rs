macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! SipHasher128 {
    () => {
        deps!();
        # [derive (Debug , Clone)] # [repr (C)] pub struct SipHasher128 { nbuf : usize , buf : [MaybeUninit < u64 > ; BUFFER_WITH_SPILL_CAPACITY] , state : State , processed : usize , }
    };
}

SipHasher128!();