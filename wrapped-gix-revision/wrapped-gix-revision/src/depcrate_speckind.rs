// Generated macro for Kind (enum)
macro_rules! Depcrate_specKind {
() => {
// Module: crate::spec
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " How to interpret a revision specification, or `revspec`."] # [derive (Default , Debug , Copy , Clone , PartialOrd , PartialEq , Ord , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Kind { # [doc = " Include commits reachable from this revision, the default when parsing revision `a` for example, i.e. `a` and its ancestors."] # [doc = " Example: `a`."] # [default] IncludeReachable , # [doc = " Exclude commits reachable from this revision, i.e. `a` and its ancestors. Example: `^a`."] ExcludeReachable , # [doc = " Every commit that is reachable from `b` but not from `a`. Example: `a..b`."] RangeBetween , # [doc = " Every commit reachable through either `a` or `b` but no commit that is reachable by both. Example: `a...b`."] ReachableToMergeBase , # [doc = " Include every commit of all parents of `a`, but not `a` itself. Example: `a^@`."] IncludeReachableFromParents , # [doc = " Exclude every commit of all parents of `a`, but not `a` itself. Example: `a^!`."] ExcludeReachableFromParents , }
};
}
