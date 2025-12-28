macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! Hasher {
    () => {
        deps!();
        # [derive (Clone)] # [doc = " Represents an in-progress CRC32 computation."] pub struct Hasher { amount : u64 , state : State , }
    };
}

Hasher!()