macro_rules! deps {
    () => {
        GateReason!();
    };
}

macro_rules! UnstableAbi {
    () => {
        deps!();
        pub struct UnstableAbi { abi : ExternAbi , feature : Symbol , explain : GateReason , }
    };
}

UnstableAbi!()