macro_rules! deps {
    () => {
        State!();
        Ref!();
    };
}

macro_rules! Reading {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Reading { state : object :: Ref < State > , }
    };
}

Reading!()