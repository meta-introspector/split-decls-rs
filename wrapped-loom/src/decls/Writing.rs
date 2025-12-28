macro_rules! deps {
    () => {
        State!();
        Ref!();
    };
}

macro_rules! Writing {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Writing { state : object :: Ref < State > , }
    };
}

Writing!()