// Generated macro for test_basic_mime (function)
macro_rules! Depcrate_mimetest_basic_mime {
() => {
// Module: crate::mime
// Provides: {"test_basic_mime"}
// Dependencies: {}
# [test] fn test_basic_mime () { let mime = Mime :: new ("text" , "plain") ; assert ! (mime . matches ("text" , "plain")) ; let cloned = mime . clone () ; assert ! (cloned . matches ("text" , "plain")) ; let mime = Mime { type_ : "text" . into () , subtype : "html" . into () , parameters : vec ! [("one" . into () , "two" . into ())] , } ; assert ! (mime . matches ("text" , "html")) ; }
};
}
