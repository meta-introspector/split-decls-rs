// Generated macro for test (module)
macro_rules! Depcrate_student_ttest {
() => {
// Module: crate::student_t
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_t () { let t = StudentT :: new (11.0) . unwrap () ; let mut rng = crate :: test :: rng (205) ; for _ in 0 .. 1000 { t . sample (& mut rng) ; } } # [test] fn student_t_distributions_can_be_compared () { assert_eq ! (StudentT :: new (1.0) , StudentT :: new (1.0)) ; } }
};
}
