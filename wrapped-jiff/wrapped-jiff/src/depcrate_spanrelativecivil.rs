// Generated macro for RelativeCivil (struct)
macro_rules! Depcrate_spanRelativeCivil {
() => {
// Module: crate::span
// Provides: {"RelativeCivil"}
// Dependencies: {}
# [doc = " A wrapper around a civil datetime and a timestamp corresponding to that"] # [doc = " civil datetime in UTC."] # [doc = ""] # [doc = " Haphazardly interpreting a civil datetime in UTC is an odd and *usually*"] # [doc = " incorrect thing to do. But the way we use it here is basically just to give"] # [doc = " it an \"anchoring\" point such that we can represent it using a single"] # [doc = " integer for rounding purposes. It is only used in a context *relative* to"] # [doc = " another civil datetime interpreted in UTC. In this fashion, the selection"] # [doc = " of UTC specifically doesn't really matter. We could use any time zone."] # [doc = " (Although, it must be a time zone without any transitions, otherwise we"] # [doc = " could wind up with time zone aware results in a context where that would"] # [doc = " be unexpected since this is civil time.)"] # [derive (Clone , Copy , Debug)] struct RelativeCivil { datetime : DateTime , timestamp : Timestamp , }
};
}
