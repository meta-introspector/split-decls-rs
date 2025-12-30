// Generated macro for test_to_string (function)
macro_rules! Depcrate_content_jsontest_to_string {
() => {
// Module: crate::content::json
// Provides: {"test_to_string"}
// Dependencies: {}
# [test] fn test_to_string () { let json = to_string (& Content :: Map (vec ! [(Content :: from ("environments") , Content :: Seq (vec ! [Content :: from ("development") , Content :: from ("production") ,]) ,) , (Content :: from ("cmdline") , Content :: Seq (vec ! [])) , (Content :: from ("extra") , Content :: Map (vec ! [])) ,])) ; crate :: assert_snapshot ! (& json , @ r#"{"environments":["development","production"],"cmdline":[],"extra":{}}"#) ; }
};
}
