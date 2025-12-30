// Generated macro for xml_detection (function)
macro_rules! Depcrate_serde_testsxml_detection {
() => {
// Module: crate::serde_tests
// Provides: {"xml_detection"}
// Dependencies: {}
# [test] fn xml_detection () { for bom in [true , false] { for whitespace in [true , false] { for decl in [true , false] { for comment in [true , false] { for doctype in [true , false] { assert ! (try_parse_xml (bom , whitespace , decl , comment , doctype) , "bom={bom}, whitespace={whitespace}, decl={decl}, comment={comment}, doctype={doctype}") ; } } } } } }
};
}
