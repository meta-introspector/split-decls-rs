// Generated macro for test_easter (function)
macro_rules! Depcrate_gregoriantest_easter {
() => {
// Module: crate::gregorian
// Provides: {"test_easter"}
// Dependencies: {}
# [test] fn test_easter () { for (y , m , d) in [(2021 , 4 , 4) , (2022 , 4 , 17) , (2023 , 4 , 9) , (2024 , 3 , 31) , (2025 , 4 , 20) , (2026 , 4 , 5) , (2027 , 3 , 28) , (2028 , 4 , 16) , (2029 , 4 , 1) ,] { assert_eq ! (easter (y) , fixed_from_gregorian (y , m , d)) ; } }
};
}
