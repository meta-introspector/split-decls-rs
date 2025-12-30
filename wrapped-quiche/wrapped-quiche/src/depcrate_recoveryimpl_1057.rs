// Generated macro for impl_1057 (impl)
macro_rules! Depcrate_recoveryimpl_1057 {
() => {
// Module: crate::recovery
// Provides: {"impl_1057"}
// Dependencies: {}
impl ReleaseTime { # [doc = " Add the specific delay to the current time"] # [allow (dead_code)] fn inc (& mut self , delay : Duration) { match self { ReleaseTime :: Immediate => { } , ReleaseTime :: At (time) => * time += delay , } } # [doc = " Set the time to the later of two times"] # [allow (dead_code)] fn set_max (& mut self , other : Instant) { match self { ReleaseTime :: Immediate => * self = ReleaseTime :: At (other) , ReleaseTime :: At (time) => * self = ReleaseTime :: At (other . max (* time)) , } } }
};
}
