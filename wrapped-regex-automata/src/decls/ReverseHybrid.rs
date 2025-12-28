macro_rules! deps {
    () => {
        ReverseHybridEngine!();
    };
}

macro_rules! ReverseHybrid {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ReverseHybrid (Option < ReverseHybridEngine >) ;
    };
}

ReverseHybrid!();