// Generated macro for SpanZoneOffsetHours (type)
macro_rules! Depcrate_util_tSpanZoneOffsetHours {
() => {
// Module: crate::util::t
// Provides: {"SpanZoneOffsetHours"}
// Dependencies: {}
# [doc = " The number of hours allowed in a time zone offset."] # [doc = ""] # [doc = " This number was somewhat arbitrarily chosen. In part because it's"] # [doc = " bigger than any current offset by a wide margin, and in part because"] # [doc = " POSIX `TZ` strings require the ability to store offsets in the range"] # [doc = " `-24:59:59..=25:59:59`. Note though that we make the range a little bigger"] # [doc = " with `-25:59:59..=25:59:59` so that negating an offset always produces a"] # [doc = " valid offset."] # [doc = ""] # [doc = " Note that RFC 8536 actually allows offsets to be much bigger, namely, in"] # [doc = " the range `(-2^31, 2^31)`, where both ends are _exclusive_ (`-2^31` is"] # [doc = " explicitly disallowed, and `2^31` overflows a signed 32-bit integer). But"] # [doc = " RFC 8536 does say that it *should* be in the range `[-89999, 93599]`, which"] # [doc = " matches POSIX. In order to keep our offset small, we stick roughly to what"] # [doc = " POSIX requires."] pub (crate) type SpanZoneOffsetHours = ri8 < - 25 , 25 > ;
};
}
