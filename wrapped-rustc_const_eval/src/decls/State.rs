macro_rules! deps {
    () => {
        FlowSensitiveAnalysis!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] # [doc = " The state for the `FlowSensitiveAnalysis` dataflow analysis. This domain is likely homogeneous,"] # [doc = " and has a big size, so we use a bitset that can be sparse (c.f. issue #134404)."] pub (super) struct State { # [doc = " Describes whether a local contains qualif."] pub qualif : MixedBitSet < Local > , # [doc = " Describes whether a local's address escaped and it might become qualified as a result an"] # [doc = " indirect mutation."] pub borrow : MixedBitSet < Local > , }
    };
}

State!();