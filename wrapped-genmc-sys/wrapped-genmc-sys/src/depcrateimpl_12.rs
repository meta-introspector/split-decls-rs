// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl FromStr for SchedulePolicy { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "wf" => SchedulePolicy :: WF , "wfr" => SchedulePolicy :: WFR , "arbitrary" | "random" => SchedulePolicy :: Arbitrary , "ltr" => SchedulePolicy :: LTR , _ => return Err ("invalid scheduling policy") , }) } }
};
}
