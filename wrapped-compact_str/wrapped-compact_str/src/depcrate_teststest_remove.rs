// Generated macro for test_remove (function)
macro_rules! Depcrate_teststest_remove {
() => {
// Module: crate::tests
// Provides: {"test_remove"}
// Dependencies: {}
# [test] fn test_remove () { let mut control = String :: from ("🦄🦀hello🎶world🇺🇸") ; let mut compact = CompactString :: from (& control) ; assert_eq ! (control . remove (0) , compact . remove (0)) ; assert_eq ! (control , compact) ; assert_eq ! (compact , "🦀hello🎶world🇺🇸") ; let music_idx = control . char_indices () . find (| (_idx , c) | * c == '🎶') . map (| (idx , _c) | idx) . unwrap () ; assert_eq ! (control . remove (music_idx) , compact . remove (music_idx)) ; assert_eq ! (control , compact) ; assert_eq ! (compact , "🦀helloworld🇺🇸") ; }
};
}
