// Generated macro for test_easter (function)
macro_rules! Depcrate_juliantest_easter {
() => {
// Module: crate::julian
// Provides: {"test_easter"}
// Dependencies: {}
# [test] fn test_easter () { for (y , m , d) in [(2021 , 5 , 2) , (2022 , 4 , 24) , (2023 , 4 , 16) , (2024 , 5 , 5) , (2025 , 4 , 20) , (2026 , 4 , 12) , (2027 , 5 , 2) , (2028 , 4 , 16) , (2029 , 4 , 8) ,] { assert_eq ! (easter (y) , crate :: gregorian :: fixed_from_gregorian (y , m , d)) ; } }
};
}
