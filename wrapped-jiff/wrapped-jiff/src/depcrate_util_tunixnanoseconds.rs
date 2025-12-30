// Generated macro for UnixNanoseconds (type)
macro_rules! Depcrate_util_tUnixNanoseconds {
() => {
// Module: crate::util::t
// Provides: {"UnixNanoseconds"}
// Dependencies: {}
# [doc = " Like UnixSeconds, but expressed in units of nanoseconds."] pub (crate) type UnixNanoseconds = ri128 < { UnixSeconds :: MIN * NANOS_PER_SECOND . bound () } , { UnixSeconds :: MAX * NANOS_PER_SECOND . bound () + FractionalNanosecond :: MAX } , > ;
};
}
