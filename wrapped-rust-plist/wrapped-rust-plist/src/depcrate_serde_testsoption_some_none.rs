// Generated macro for option_some_none (function)
macro_rules! Depcrate_serde_testsoption_some_none {
() => {
// Module: crate::serde_tests
// Provides: {"option_some_none"}
// Dependencies: {}
# [test] fn option_some_none () { let obj : Option < Option < u32 > > = Some (None) ; let comparison = & [Event :: StartDictionary (Some (1)) , Event :: String ("None" . into ()) , Event :: String ("" . into ()) , Event :: EndCollection ,] ; assert_roundtrip (obj , comparison , true) ; }
};
}
