// Generated macro for sanity_unclaimed_ratio (function)
macro_rules! Depcrate_proto_streams_flow_controlsanity_unclaimed_ratio {
() => {
// Module: crate::proto::streams::flow_control
// Provides: {"sanity_unclaimed_ratio"}
// Dependencies: {}
# [test] # [allow (clippy :: assertions_on_constants)] fn sanity_unclaimed_ratio () { assert ! (UNCLAIMED_NUMERATOR < UNCLAIMED_DENOMINATOR) ; assert ! (UNCLAIMED_NUMERATOR >= 0) ; assert ! (UNCLAIMED_DENOMINATOR > 0) ; }
};
}
