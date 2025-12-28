macro_rules! deps {
    () => {
        Ref!();
        State!();
    };
}

macro_rules! Reading {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Reading { state : object :: Ref < State > , }
    };
}

Reading!();