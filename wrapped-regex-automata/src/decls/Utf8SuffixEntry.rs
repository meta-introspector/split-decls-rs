macro_rules! deps {
    () => {
        StateID!();
        Utf8SuffixKey!();
    };
}

macro_rules! Utf8SuffixEntry {
    () => {
        deps!();
        # [doc = " An entry in this map."] # [derive (Clone , Debug , Default)] struct Utf8SuffixEntry { # [doc = " The version of the map used to produce this entry. If this entry's"] # [doc = " version does not match the current version of the map, then the map"] # [doc = " should behave as if this entry does not exist."] version : u16 , # [doc = " The key, which consists of a transition in a particular state."] key : Utf8SuffixKey , # [doc = " The identifier that the transition in the key maps to."] val : StateID , }
    };
}

Utf8SuffixEntry!()