// Generated macro for AutoFinisher (struct)
macro_rules! DepcrateAutoFinisher {
() => {
// Module: crate
// Provides: {"AutoFinisher"}
// Dependencies: {}
# [doc = " A wrapper around a writer that finishes the stream on drop."] # [allow (private_bounds)] pub struct AutoFinisher < T : AutoFinish > (Option < T >) ;
};
}
