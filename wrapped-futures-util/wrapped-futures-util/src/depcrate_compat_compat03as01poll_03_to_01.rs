// Generated macro for poll_03_to_01 (function)
macro_rules! Depcrate_compat_compat03as01poll_03_to_01 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"poll_03_to_01"}
// Dependencies: {}
fn poll_03_to_01 < T , E > (x : task03 :: Poll < Result < T , E > >) -> Result < Async01 < T > , E > { match x ? { task03 :: Poll :: Ready (t) => Ok (Async01 :: Ready (t)) , task03 :: Poll :: Pending => Ok (Async01 :: NotReady) , } }
};
}
