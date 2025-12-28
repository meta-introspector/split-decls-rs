macro_rules! deps {
    () => {
        Ref!();
        State!();
    };
}

macro_rules! Writing {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Writing { state : object :: Ref < State > , }
    };
}

Writing!();