// Generated macro for test_color (function)
macro_rules! Depcrate_core_colortest_color {
() => {
// Module: crate::core::color
// Provides: {"test_color"}
// Dependencies: {}
# [test] fn test_color () { let color = Color :: new (0x56_ff_00_7f) ; assert_eq ! (color . to_web_color () , "#56ff007f") ; let color = Color :: from_name ("coral") ; assert_eq ! (color . unwrap () . to_web_color () , "#ff7f50ff") ; let color = Color :: from_name ("#112233") ; assert_eq ! (color . unwrap () . to_web_color () , "#112233ff") ; let color = Color :: from_name ("#112233FA") ; assert_eq ! (color . unwrap () . to_web_color () , "#112233fa") ; }
};
}
