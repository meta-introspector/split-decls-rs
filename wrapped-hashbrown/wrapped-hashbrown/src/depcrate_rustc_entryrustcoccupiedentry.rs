// Generated macro for RustcOccupiedEntry (struct)
macro_rules! Depcrate_rustc_entryRustcOccupiedEntry {
() => {
// Module: crate::rustc_entry
// Provides: {"RustcOccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in a `HashMap`."] # [doc = " It is part of the [`RustcEntry`] enum."] # [doc = ""] # [doc = " [`RustcEntry`]: enum.RustcEntry.html"] pub struct RustcOccupiedEntry < 'a , K , V , A = Global > where A : Allocator , { elem : Bucket < (K , V) > , table : & 'a mut RawTable < (K , V) , A > , }
};
}
