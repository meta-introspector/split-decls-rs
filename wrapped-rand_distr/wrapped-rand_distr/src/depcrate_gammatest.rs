// Generated macro for test (module)
macro_rules! Depcrate_gammatest {
() => {
// Module: crate::gamma
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn gamma_distributions_can_be_compared () { assert_eq ! (Gamma :: new (1.0 , 2.0) , Gamma :: new (1.0 , 2.0)) ; } }
};
}
