// Generated macro for test_splayed_counter (function)
macro_rules! Depcrate_matmultest_splayed_counter {
() => {
// Module: crate::matmul
// Provides: {"test_splayed_counter"}
// Dependencies: {}
# [test] fn test_splayed_counter () { let bits : Vec < usize > = SplayedBitsCounter :: new (64) . collect () ; assert_eq ! (vec ! [0b0 , 0b1 , 0b100 , 0b101 , 0b10000 , 0b10001 , 0b10100 , 0b10101] , bits) ; }
};
}
