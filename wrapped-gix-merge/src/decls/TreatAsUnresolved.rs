macro_rules! deps {
    () => {
        ContentMerge!();
    };
}

macro_rules! TreatAsUnresolved {
    () => {
        deps!();
        # [doc = " Determine what should be considered an unresolved conflict."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct TreatAsUnresolved { # [doc = " Determine which content merges should be considered unresolved."] pub content_merge : treat_as_unresolved :: ContentMerge , # [doc = " Determine which tree merges should be considered unresolved."] pub tree_merge : treat_as_unresolved :: TreeMerge , }
    };
}

TreatAsUnresolved!()