macro_rules! deps {
    () => {
        StateID!();
        NFA!();
    };
}

macro_rules! Transition {
    () => {
        deps!();
        # [doc = " A single transition in a non-contiguous NFA."] # [derive (Clone , Copy , Default)] # [repr (packed)] pub (crate) struct Transition { byte : u8 , next : StateID , link : StateID , }
    };
}

Transition!();