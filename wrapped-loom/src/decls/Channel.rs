macro_rules! deps {
    () => {
        Ref!();
        State!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Channel { state : object :: Ref < State > , }
    };
}

Channel!()