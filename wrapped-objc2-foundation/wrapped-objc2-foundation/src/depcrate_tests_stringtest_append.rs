// Generated macro for test_append (function)
macro_rules! Depcrate_tests_stringtest_append {
() => {
// Module: crate::tests::string
// Provides: {"test_append"}
// Dependencies: {}
# [test] # [cfg (feature = "NSPathUtilities")] fn test_append () { let error_tag = NSString :: from_str ("Error: ") ; let error_string = NSString :: from_str ("premature end of file.") ; let error_message = error_tag . stringByAppendingString (& error_string) ; assert_eq ! (error_message , NSString :: from_str ("Error: premature end of file.")) ; let extension = NSString :: from_str ("scratch.tiff") ; assert_eq ! (NSString :: from_str ("/tmp") . stringByAppendingPathComponent (& extension) , NSString :: from_str ("/tmp/scratch.tiff")) ; assert_eq ! (NSString :: from_str ("/tmp/") . stringByAppendingPathComponent (& extension) , NSString :: from_str ("/tmp/scratch.tiff")) ; assert_eq ! (NSString :: from_str ("/") . stringByAppendingPathComponent (& extension) , NSString :: from_str ("/scratch.tiff")) ; assert_eq ! (NSString :: from_str ("") . stringByAppendingPathComponent (& extension) , NSString :: from_str ("scratch.tiff")) ; }
};
}
