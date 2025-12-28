macro_rules! deps {
    () => {
        Overlap!();
    };
}

macro_rules! PlaceConflictBias {
    () => {
        deps!();
        # [doc = " When checking if a place conflicts with another place, this enum is used to influence decisions"] # [doc = " where a place might be equal or disjoint with another place, such as if `a[i] == a[j]`."] # [doc = " `PlaceConflictBias::Overlap` would bias toward assuming that `i` might equal `j` and that these"] # [doc = " places overlap. `PlaceConflictBias::NoOverlap` assumes that for the purposes of the predicate"] # [doc = " being run in the calling context, the conservative choice is to assume the compared indices"] # [doc = " are disjoint (and therefore, do not overlap)."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum PlaceConflictBias { Overlap , NoOverlap , }
    };
}

PlaceConflictBias!()