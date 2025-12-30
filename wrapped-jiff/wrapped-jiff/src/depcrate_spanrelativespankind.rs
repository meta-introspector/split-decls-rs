// Generated macro for RelativeSpanKind (enum)
macro_rules! Depcrate_spanRelativeSpanKind {
() => {
// Module: crate::span
// Provides: {"RelativeSpanKind"}
// Dependencies: {}
# [doc = " A civil or zoned datetime range of time."] # [derive (Clone , Debug)] enum RelativeSpanKind < 'a > { Civil { start : RelativeCivil , end : RelativeCivil } , Zoned { start : RelativeZoned < 'a > , end : RelativeZoned < 'a > } , }
};
}
