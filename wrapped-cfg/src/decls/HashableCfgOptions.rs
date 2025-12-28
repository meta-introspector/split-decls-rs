macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! HashableCfgOptions {
    () => {
        deps!();
        # [doc = " A `CfgOptions` that implements `Hash`, for the sake of hashing only."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct HashableCfgOptions { _enabled : Box < [CfgAtom] > , }
    };
}

HashableCfgOptions!()