// Generated macro for macro_2359 (macro)
macro_rules! Depcrate_ntrtlmacro_2359 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2359"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCutoverTimeToSystemTime (CutoverTime : PTIME_FIELDS , SystemTime : PLARGE_INTEGER , CurrentSystemTime : PLARGE_INTEGER , ThisYear : BOOLEAN ,) -> BOOLEAN ; fn RtlSystemTimeToLocalTime (SystemTime : PLARGE_INTEGER , LocalTime : PLARGE_INTEGER ,) -> NTSTATUS ; fn RtlLocalTimeToSystemTime (LocalTime : PLARGE_INTEGER , SystemTime : PLARGE_INTEGER ,) -> NTSTATUS ; fn RtlTimeToElapsedTimeFields (Time : PLARGE_INTEGER , TimeFields : PTIME_FIELDS ,) ; fn RtlTimeToTimeFields (Time : PLARGE_INTEGER , TimeFields : PTIME_FIELDS ,) ; fn RtlTimeFieldsToTime (TimeFields : PTIME_FIELDS , Time : PLARGE_INTEGER ,) -> BOOLEAN ; fn RtlTimeToSecondsSince1980 (Time : PLARGE_INTEGER , ElapsedSeconds : PULONG ,) -> BOOLEAN ; fn RtlSecondsSince1980ToTime (ElapsedSeconds : ULONG , Time : PLARGE_INTEGER ,) ; fn RtlTimeToSecondsSince1970 (Time : PLARGE_INTEGER , ElapsedSeconds : PULONG ,) -> BOOLEAN ; fn RtlSecondsSince1970ToTime (ElapsedSeconds : ULONG , Time : PLARGE_INTEGER ,) ; } }
};
}
