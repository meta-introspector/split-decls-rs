// Generated macro for Entry (struct)
macro_rules! Depcrate_baseEntry {
() => {
// Module: crate::base
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry in a skip list, protected by a `Guard`."] # [doc = ""] # [doc = " The lifetimes of the key and value are the same as that of the `Guard`"] # [doc = " used when creating the `Entry` (`'g`). This lifetime is also constrained to"] # [doc = " not outlive the `SkipList`."] pub struct Entry < 'a : 'g , 'g , K , V > { parent : & 'a SkipList < K , V > , node : & 'g Node < K , V > , guard : & 'g Guard , }
};
}
