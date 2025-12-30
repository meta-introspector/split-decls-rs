// Generated macro for test_markers_for_bin (function)
macro_rules! Depcrate_datagentest_markers_for_bin {
() => {
// Module: crate::datagen
// Provides: {"test_markers_for_bin"}
// Dependencies: {}
# [test] fn test_markers_for_bin () { assert_eq ! (markers_for_bin (include_bytes ! ("../tests/data/tutorial_buffer.wasm")) . unwrap () , [crate :: datetime :: provider :: neo :: DayPeriodNamesV1 :: INFO , crate :: datetime :: provider :: neo :: DatetimeNamesMonthGregorianV1 :: INFO , crate :: datetime :: provider :: neo :: DatetimeNamesYearGregorianV1 :: INFO , crate :: datetime :: provider :: neo :: DatetimePatternsGlueV1 :: INFO , crate :: datetime :: provider :: DatetimePatternsDateGregorianV1 :: INFO , crate :: datetime :: provider :: DatetimePatternsTimeV1 :: INFO , crate :: decimal :: provider :: DecimalSymbolsV1 :: INFO , crate :: decimal :: provider :: DecimalDigitsV1 :: INFO ,] . into_iter () . collect () ,) ; }
};
}
