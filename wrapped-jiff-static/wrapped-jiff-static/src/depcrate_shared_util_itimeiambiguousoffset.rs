// Generated macro for IAmbiguousOffset (enum)
macro_rules! Depcrate_shared_util_itimeIAmbiguousOffset {
() => {
// Module: crate::shared::util::itime
// Provides: {"IAmbiguousOffset"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , PartialEq)] pub (crate) enum IAmbiguousOffset { Unambiguous { offset : IOffset } , Gap { before : IOffset , after : IOffset } , Fold { before : IOffset , after : IOffset } , }
};
}
