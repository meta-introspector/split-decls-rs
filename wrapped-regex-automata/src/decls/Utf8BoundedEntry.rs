macro_rules! deps {
    () => {
        StateID!();
        NFA!();
        Transition!();
    };
}

macro_rules! Utf8BoundedEntry {
    () => {
        deps!();
        # [doc = " An entry in this map."] # [derive (Clone , Debug , Default)] struct Utf8BoundedEntry { # [doc = " The version of the map used to produce this entry. If this entry's"] # [doc = " version does not match the current version of the map, then the map"] # [doc = " should behave as if this entry does not exist."] version : u16 , # [doc = " The key, which is a sorted sequence of non-overlapping NFA transitions."] key : Vec < Transition > , # [doc = " The state ID corresponding to the state containing the transitions in"] # [doc = " this entry."] val : StateID , }
    };
}

Utf8BoundedEntry!()