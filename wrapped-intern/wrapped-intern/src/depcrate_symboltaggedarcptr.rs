// Generated macro for TaggedArcPtr (struct)
macro_rules! Depcrate_symbolTaggedArcPtr {
() => {
// Module: crate::symbol
// Provides: {"TaggedArcPtr"}
// Dependencies: {}
# [doc = " A pointer that points to a pointer to a `str`, it may be backed as a `&'static &'static str` or"] # [doc = " `Arc<Box<str>>` but its size is that of a thin pointer. The active variant is encoded as a tag"] # [doc = " in the LSB of the alignment niche."] # [derive (PartialEq , Eq , Hash , Copy , Clone , Debug)] struct TaggedArcPtr { packed : NonNull < * const str > , }
};
}
