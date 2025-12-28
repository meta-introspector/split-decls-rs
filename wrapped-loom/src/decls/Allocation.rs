macro_rules! deps {
    () => {
        Ref!();
        State!();
    };
}

macro_rules! Allocation {
    () => {
        deps!();
        # [doc = " Tracks an allocation"] # [derive (Debug)] pub (crate) struct Allocation { state : object :: Ref < State > , }
    };
}

Allocation!()