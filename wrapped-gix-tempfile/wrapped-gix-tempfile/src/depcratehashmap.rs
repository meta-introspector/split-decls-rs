// Generated macro for HashMap (type)
macro_rules! DepcrateHashMap {
() => {
// Module: crate
// Provides: {"HashMap"}
// Dependencies: {}
# [cfg (not (feature = "hp-hashmap"))] type HashMap < K , V > = hashmap :: Concurrent < K , V > ;
};
}
