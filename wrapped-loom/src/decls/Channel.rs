macro_rules! deps {
    () => {
        State!();
        Ref!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Channel { state : object :: Ref < State > , }
    };
}

Channel!();