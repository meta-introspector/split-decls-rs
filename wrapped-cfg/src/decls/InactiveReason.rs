macro_rules! InactiveReason {
    () => {
        pub struct InactiveReason { enabled : Vec < CfgAtom > , disabled : Vec < CfgAtom > , }
    };
}

InactiveReason!()