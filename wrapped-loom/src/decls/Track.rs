macro_rules! deps {
    () => {
        Allocation!();
    };
}

macro_rules! Track {
    () => {
        deps!();
        # [doc = " Track allocations, detecting leaks"] # [derive (Debug)] pub struct Track < T > { value : T , # [doc = " Drop guard tracking the allocation's lifetime."] _obj : rt :: Allocation , }
    };
}

Track!()