// Generated macro for impl_79 (impl)
macro_rules! Depcrate_datetimeimpl_79 {
() => {
// Module: crate::datetime
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "time")] impl TryFrom < DateTime > for PrimitiveDateTime { type Error = Error ; fn try_from (time : DateTime) -> Result < PrimitiveDateTime > { let month = time . month () . try_into () ? ; let date = time :: Date :: from_calendar_date (i32 :: from (time . year ()) , month , time . day ()) ? ; let time = time :: Time :: from_hms (time . hour () , time . minutes () , time . seconds ()) ? ; Ok (PrimitiveDateTime :: new (date , time)) } }
};
}
