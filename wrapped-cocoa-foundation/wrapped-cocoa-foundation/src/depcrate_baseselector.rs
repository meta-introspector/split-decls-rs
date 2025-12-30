// Generated macro for selector (function)
macro_rules! Depcrate_baseselector {
() => {
// Module: crate::base
// Provides: {"selector"}
// Dependencies: {}
# [doc = " A convenience method to convert the name of a selector to the selector object."] # [inline] pub fn selector (name : & str) -> SEL { runtime :: Sel :: register (name) }
};
}
