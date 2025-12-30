// Generated macro for ShortBoxSliceIntoIterInner (enum)
macro_rules! Depcrate_shortvecShortBoxSliceIntoIterInner {
() => {
// Module: crate::shortvec
// Provides: {"ShortBoxSliceIntoIterInner"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum ShortBoxSliceIntoIterInner < T > { ZeroOne (Option < T >) , # [cfg (feature = "alloc")] Multi (alloc :: vec :: IntoIter < T >) , # [cfg (not (feature = "alloc"))] Two (core :: array :: IntoIter < T , 2 >) , }
};
}
