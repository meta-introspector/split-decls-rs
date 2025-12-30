// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Date { fn months_since (self , other : Date) -> Option < u32 > { let self_chrono = Utc . with_ymd_and_hms (self . year . try_into () . unwrap () , self . month , 1 , 0 , 0 , 0) . unwrap () ; let other_chrono = Utc . with_ymd_and_hms (other . year . try_into () . unwrap () , other . month , 1 , 0 , 0 , 0) . unwrap () ; let duration_since = self_chrono . signed_duration_since (other_chrono) ; let months_since = duration_since . num_days () / 30 ; if months_since < 0 { None } else { Some (months_since . try_into () . unwrap ()) } } }
};
}
