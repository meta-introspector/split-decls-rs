// Generated macro for UnixMicroseconds (type)
macro_rules! Depcrate_util_tUnixMicroseconds {
() => {
// Module: crate::util::t
// Provides: {"UnixMicroseconds"}
// Dependencies: {}
# [doc = " Like UnixSeconds, but expressed in units of microseconds."] pub (crate) type UnixMicroseconds = ri64 < { UnixSeconds :: MIN * MICROS_PER_SECOND . bound () } , { (UnixSeconds :: MAX * MICROS_PER_SECOND . bound ()) + (FractionalNanosecond :: MAX / NANOS_PER_MICRO . bound ()) } , > ;
};
}
