macro_rules! deps {
    () => {
        NFA!();
        StateID!();
    };
}

macro_rules! Utf8SuffixKey {
    () => {
        deps!();
        # [doc = " A key that uniquely identifies an NFA state. It is a triple that represents"] # [doc = " a transition from one state for a particular byte range."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct Utf8SuffixKey { pub from : StateID , pub start : u8 , pub end : u8 , }
    };
}

Utf8SuffixKey!();