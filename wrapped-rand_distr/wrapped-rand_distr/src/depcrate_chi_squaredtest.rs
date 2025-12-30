// Generated macro for test (module)
macro_rules! Depcrate_chi_squaredtest {
() => {
// Module: crate::chi_squared
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_chi_squared_one () { let chi = ChiSquared :: new (1.0) . unwrap () ; let mut rng = crate :: test :: rng (201) ; for _ in 0 .. 1000 { chi . sample (& mut rng) ; } } # [test] fn test_chi_squared_small () { let chi = ChiSquared :: new (0.5) . unwrap () ; let mut rng = crate :: test :: rng (202) ; for _ in 0 .. 1000 { chi . sample (& mut rng) ; } } # [test] fn test_chi_squared_large () { let chi = ChiSquared :: new (30.0) . unwrap () ; let mut rng = crate :: test :: rng (203) ; for _ in 0 .. 1000 { chi . sample (& mut rng) ; } } # [test] # [should_panic] fn test_chi_squared_invalid_dof () { ChiSquared :: new (- 1.0) . unwrap () ; } # [test] fn gamma_distributions_can_be_compared () { assert_eq ! (Gamma :: new (1.0 , 2.0) , Gamma :: new (1.0 , 2.0)) ; } # [test] fn chi_squared_distributions_can_be_compared () { assert_eq ! (ChiSquared :: new (1.0) , ChiSquared :: new (1.0)) ; } }
};
}
