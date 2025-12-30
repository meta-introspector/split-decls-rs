// Generated macro for impl_2064 (impl)
macro_rules! Depcrate_compat_compat01as03impl_2064 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"impl_2064"}
// Dependencies: {}
impl < Fut : Future01 > Future03 for Compat01As03 < Fut > { type Output = Result < Fut :: Item , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> task03 :: Poll < Self :: Output > { poll_01_to_03 (self . in_notify (cx , Future01 :: poll)) } }
};
}
