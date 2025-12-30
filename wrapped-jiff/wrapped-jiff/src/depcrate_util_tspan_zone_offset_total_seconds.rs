// Generated macro for SPAN_ZONE_OFFSET_TOTAL_SECONDS (const)
macro_rules! Depcrate_util_tSPAN_ZONE_OFFSET_TOTAL_SECONDS {
() => {
// Module: crate::util::t
// Provides: {"SPAN_ZONE_OFFSET_TOTAL_SECONDS"}
// Dependencies: {}
# [doc = " The max number of seconds that can be expressed in a time zone offset."] # [doc = ""] # [doc = " This is computed here based on the span offset types below for convenience"] # [doc = " use in the `SpanZoneOffset` definition above."] const SPAN_ZONE_OFFSET_TOTAL_SECONDS : i128 = (SpanZoneOffsetHours :: MAX * 60 * 60) + (SpanZoneOffsetMinutes :: MAX * 60) + SpanZoneOffsetSeconds :: MAX ;
};
}
