// Generated macro for test_collect (function)
macro_rules! Depcrate_teststest_collect {
() => {
// Module: crate::tests
// Provides: {"test_collect"}
// Dependencies: {}
# [test] fn test_collect () { const VALUES : & [& str] = & ["foo" , "bar" , "baz"] ; assert_eq ! (VALUES . iter () . copied () . map (Cow :: Borrowed) . collect ::< CompactString > () , "foobarbaz" ,) ; assert_eq ! (VALUES . iter () . copied () . map (| s | Cow :: Owned (s . into ())) . collect ::< CompactString > () , "foobarbaz" ,) ; assert_eq ! (VALUES . iter () . copied () . map (Box ::< str >:: from) . collect ::< CompactString > () , "foobarbaz" ,) ; assert_eq ! (VALUES . iter () . copied () . map (CompactString :: from) . collect ::< String > () , "foobarbaz" ,) ; assert_eq ! (VALUES . iter () . copied () . map (CompactString :: from) . collect ::< Cow <'_ , str >> () , "foobarbaz" ,) ; assert_eq ! (VALUES . iter () . copied () . flat_map (| s | s . chars ()) . collect ::< Cow <'_ , str >> () , "foobarbaz" ,) ; }
};
}
