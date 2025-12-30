// Generated macro for UnixMilliseconds (type)
macro_rules! Depcrate_util_tUnixMilliseconds {
() => {
// Module: crate::util::t
// Provides: {"UnixMilliseconds"}
// Dependencies: {}
# [doc = " Like UnixSeconds, but expressed in units of milliseconds."] pub (crate) type UnixMilliseconds = ri64 < { UnixSeconds :: MIN * MILLIS_PER_SECOND . bound () } , { (UnixSeconds :: MAX * MILLIS_PER_SECOND . bound ()) + (FractionalNanosecond :: MAX / NANOS_PER_MILLI . bound ()) } , > ;
};
}
