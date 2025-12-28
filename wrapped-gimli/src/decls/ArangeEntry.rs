macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! ArangeEntry {
    () => {
        deps!();
        # [doc = " A single parsed arange."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct ArangeEntry { range : Range , length : u64 , }
    };
}

ArangeEntry!();