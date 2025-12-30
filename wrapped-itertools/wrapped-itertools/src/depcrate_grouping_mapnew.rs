// Generated macro for new (function)
macro_rules! Depcrate_grouping_mapnew {
() => {
// Module: crate::grouping_map
// Provides: {"new"}
// Dependencies: {}
# [doc = " Creates a new `GroupingMap` from `iter`"] pub fn new < I , K , V > (iter : I) -> GroupingMap < I > where I : Iterator < Item = (K , V) > , K : Hash + Eq , { GroupingMap { iter } }
};
}
