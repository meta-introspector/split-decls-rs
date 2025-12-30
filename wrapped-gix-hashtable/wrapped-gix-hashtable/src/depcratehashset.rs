// Generated macro for HashSet (type)
macro_rules! DepcrateHashSet {
() => {
// Module: crate
// Provides: {"HashSet"}
// Dependencies: {}
# [doc = " A `HashSet` for usage with keys that are already robust hashes (like an `ObjectId`)."] # [doc = " The first `8` bytes of the hash are used as the `HashMap` hash"] pub type HashSet < T = ObjectId > = hashbrown :: HashSet < T , hash :: Builder > ;
};
}
