macro_rules! deps {
    () => {
        ADDRESS_MODE!();
    };
}

macro_rules! ADDRESS64 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct ADDRESS64 { pub Offset : u64 , pub Segment : u16 , pub Mode : ADDRESS_MODE , }
    };
}

ADDRESS64!()