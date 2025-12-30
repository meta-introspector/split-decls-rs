// Generated macro for test (module)
macro_rules! Depcrate_scoringtest {
() => {
// Module: crate::scoring
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: slope_stddev ; # [test] fn stddev_negative_on_linear () { let vec = vec ! [(10.0 , 100.0) , (20.0 , 201.0) , (30.0 , 299.5) , (40.0 , 385.0) , (50.0 , 510.0) ,] ; let test_result = slope_stddev (& vec) ; println ! ("Score = {}" , test_result . 0) ; assert ! (test_result . 1 == false) ; } # [test] fn stddev_negative_on_noisy_linear () { let vec = vec ! [(0.1 , 85.0) , (0.2 , 222.0) , (0.3 , 270.5) , (0.4 , 385.0) , (0.5 , 520.0) ,] ; let test_result = slope_stddev (& vec) ; println ! ("Score = {}" , test_result . 0) ; assert ! (test_result . 1 == false) ; } # [test] fn stddev_positive_on_quadratic () { let vec = vec ! [(0.1 , 100.0) , (0.2 , 400.0) , (0.3 , 880.0) , (0.4 , 1630.0) , (0.5 , 2440.0) ,] ; let test_result = slope_stddev (& vec) ; println ! ("Score = {}" , test_result . 0) ; assert ! (test_result . 1 == true) ; } # [test] fn stddev_positive_on_semiquadratic () { let vec = vec ! [(0.1 , 105.0) , (0.2 , 260.0) , (0.3 , 505.0) , (0.4 , 775.0) , (0.5 , 1118.0) ,] ; let test_result = slope_stddev (& vec) ; println ! ("Score = {}" , test_result . 0) ; assert ! (test_result . 1 == true) ; } }
};
}
