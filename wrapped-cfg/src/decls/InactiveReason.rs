macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! InactiveReason {
    () => {
        deps!();
        pub struct InactiveReason { enabled : Vec < CfgAtom > , disabled : Vec < CfgAtom > , }
    };
}

InactiveReason!()