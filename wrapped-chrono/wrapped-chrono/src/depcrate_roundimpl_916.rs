// Generated macro for impl_916 (impl)
macro_rules! Depcrate_roundimpl_916 {
() => {
// Module: crate::round
// Provides: {"impl_916"}
// Dependencies: {}
impl < Tz : TimeZone > DurationRound for DateTime < Tz > { type Err = RoundingError ; fn duration_round (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round (self . naive_local () , self , duration) } fn duration_trunc (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_trunc (self . naive_local () , self , duration) } fn duration_round_up (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round_up (self . naive_local () , self , duration) } }
};
}
