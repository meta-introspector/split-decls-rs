// Generated macro for poll_01_to_03 (function)
macro_rules! Depcrate_compat_compat01as03poll_01_to_03 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"poll_01_to_03"}
// Dependencies: {}
fn poll_01_to_03 < T , E > (x : Result < Async01 < T > , E >) -> task03 :: Poll < Result < T , E > > { match x ? { Async01 :: Ready (t) => task03 :: Poll :: Ready (Ok (t)) , Async01 :: NotReady => task03 :: Poll :: Pending , } }
};
}
