// Generated macro for Relation (struct)
macro_rules! DepcrateRelation {
() => {
// Module: crate
// Provides: {"Relation"}
// Dependencies: {}
# [doc = " A static, ordered list of key-value pairs."] # [doc = ""] # [doc = " A relation represents a fixed set of key-value pairs. In many places in a"] # [doc = " Datalog computation we want to be sure that certain relations are not able"] # [doc = " to vary (for example, in antijoins)."] # [derive (Clone)] pub struct Relation < Tuple : Ord > { # [doc = " Sorted list of distinct tuples."] pub elements : Vec < Tuple > , }
};
}
