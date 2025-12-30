// Generated macro for HashMap (type)
macro_rules! DepcrateHashMap {
() => {
// Module: crate
// Provides: {"HashMap"}
// Dependencies: {}
# [doc = " A `HashMap` for usage with keys that are already robust hashes (like an `ObjectId`)."] # [doc = " The first `8` bytes of the hash are used as the `HashMap` hash"] pub type HashMap < K , V > = hashbrown :: HashMap < K , V , hash :: Builder > ;
};
}
