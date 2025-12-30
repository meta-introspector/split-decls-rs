// Generated macro for test_multi_layout (function)
macro_rules! Depcrate_element_texttest_multi_layout {
() => {
// Module: crate::element::text
// Provides: {"test_multi_layout"}
// Dependencies: {}
# [cfg (all (feature = "ttf" , target_os = "linux"))] # [test] fn test_multi_layout () { use plotters_backend :: { FontFamily , FontStyle } ; let font = FontDesc :: new (FontFamily :: SansSerif , 20 as f64 , FontStyle :: Bold) ; layout_multiline_text ("öäabcde" , 40 , font , | txt | { println ! ("Got: {}" , txt) ; assert ! (txt == "öäabc" || txt == "de") ; }) ; let font = FontDesc :: new (FontFamily :: SansSerif , 20 as f64 , FontStyle :: Bold) ; layout_multiline_text ("öä" , 100 , font , | txt | { println ! ("Got: {}" , txt) ; assert_eq ! (txt , "öä") }) ; }
};
}
