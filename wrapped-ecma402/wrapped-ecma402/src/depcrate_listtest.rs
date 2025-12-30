// Generated macro for test (function)
macro_rules! Depcrate_listtest {
() => {
// Module: crate::list
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () { use ecma402_traits :: listformat :: Format ; let mut buf = String :: new () ; ListFormat :: try_new (crate :: testing :: TestLocale ("es") , Options { in_type : Type :: Conjunction , style : Style :: Long , } ,) . unwrap () . format (["Mallorca" , "Ibiza"] , & mut buf) . unwrap () ; assert_eq ! (buf , "Mallorca e Ibiza") ; }
};
}
