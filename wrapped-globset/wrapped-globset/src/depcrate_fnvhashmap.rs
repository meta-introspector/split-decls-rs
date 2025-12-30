// Generated macro for HashMap (type)
macro_rules! Depcrate_fnvHashMap {
() => {
// Module: crate::fnv
// Provides: {"HashMap"}
// Dependencies: {}
# [doc = " A convenience alias for creating a hash map with an FNV hasher."] pub (crate) type HashMap < K , V > = std :: collections :: HashMap < K , V , std :: hash :: BuildHasherDefault < Hasher > > ;
};
}
