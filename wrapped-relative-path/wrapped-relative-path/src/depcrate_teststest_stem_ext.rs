// Generated macro for test_stem_ext (function)
macro_rules! Depcrate_teststest_stem_ext {
() => {
// Module: crate::tests
// Provides: {"test_stem_ext"}
// Dependencies: {}
# [test] pub fn test_stem_ext () { t ! ("foo" , file_stem : Some ("foo") , extension : None) ; t ! ("foo." , file_stem : Some ("foo") , extension : Some ("")) ; t ! (".foo" , file_stem : Some (".foo") , extension : None) ; t ! ("foo.txt" , file_stem : Some ("foo") , extension : Some ("txt")) ; t ! ("foo.bar.txt" , file_stem : Some ("foo.bar") , extension : Some ("txt")) ; t ! ("foo.bar." , file_stem : Some ("foo.bar") , extension : Some ("")) ; t ! ("." , file_stem : None , extension : None) ; t ! (".." , file_stem : None , extension : None) ; t ! ("" , file_stem : None , extension : None) ; }
};
}
