// Generated macro for impl_23 (impl)
macro_rules! Depcrate_time_deltaimpl_23 {
() => {
// Module: crate::time_delta
// Provides: {"impl_23"}
// Dependencies: {}
impl Neg for TimeDelta { type Output = TimeDelta ; # [inline] fn neg (self) -> TimeDelta { let (secs_diff , nanos) = match self . nanos { 0 => (0 , 0) , nanos => (1 , NANOS_PER_SEC - nanos) , } ; TimeDelta { secs : - self . secs - secs_diff , nanos } } }
};
}
