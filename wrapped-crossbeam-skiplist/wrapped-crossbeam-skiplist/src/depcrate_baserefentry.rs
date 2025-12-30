// Generated macro for RefEntry (struct)
macro_rules! Depcrate_baseRefEntry {
() => {
// Module: crate::base
// Provides: {"RefEntry"}
// Dependencies: {}
# [doc = " A reference-counted entry in a skip list."] # [doc = ""] # [doc = " You *must* call `release` to free this type, otherwise the node will be"] # [doc = " leaked. This is because releasing the entry requires a `Guard`."] pub struct RefEntry < 'a , K , V > { parent : & 'a SkipList < K , V > , node : & 'a Node < K , V > , }
};
}
