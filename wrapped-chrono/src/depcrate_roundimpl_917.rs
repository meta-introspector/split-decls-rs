// Generated macro for impl_917 (impl)
macro_rules! Depcrate_roundimpl_917 {
() => {
// Module: crate::round
// Provides: {"impl_917"}
// Dependencies: {}
impl DurationRound for NaiveDateTime { type Err = RoundingError ; fn duration_round (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round (self , self , duration) } fn duration_trunc (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_trunc (self , self , duration) } fn duration_round_up (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round_up (self , self , duration) } }
};
}
