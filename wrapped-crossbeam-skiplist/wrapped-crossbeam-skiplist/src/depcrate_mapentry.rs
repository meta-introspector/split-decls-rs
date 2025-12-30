// Generated macro for Entry (struct)
macro_rules! Depcrate_mapEntry {
() => {
// Module: crate::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A reference-counted entry in a map."] pub struct Entry < 'a , K , V > { inner : ManuallyDrop < base :: RefEntry < 'a , K , V > > , }
};
}
