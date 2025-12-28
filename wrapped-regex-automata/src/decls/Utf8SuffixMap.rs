macro_rules! deps {
    () => {
        Utf8SuffixEntry!();
    };
}

macro_rules! Utf8SuffixMap {
    () => {
        deps!();
        # [doc = " A cache of suffixes used to modestly compress UTF-8 automata for large"] # [doc = " Unicode character classes."] # [derive (Clone , Debug)] pub struct Utf8SuffixMap { # [doc = " The current version of this map. Only entries with matching versions"] # [doc = " are considered during lookups. If an entry is found with a mismatched"] # [doc = " version, then the map behaves as if the entry does not exist."] version : u16 , # [doc = " The total number of entries this map can store."] capacity : usize , # [doc = " The actual entries, keyed by hash. Collisions between different states"] # [doc = " result in the old state being dropped."] map : Vec < Utf8SuffixEntry > , }
    };
}

Utf8SuffixMap!();