// Generated macro for new_test (macro)
macro_rules! Depcrate_devnew_test {
() => {
// Module: crate::dev
// Provides: {"new_test"}
// Dependencies: {}
# [doc = " Define hash function test"] # [macro_export] macro_rules ! new_test { ($ name : ident , $ hasher : ty , $ test_fn : ident $ (,) ?) => { # [test] fn $ name () { use $ crate :: dev :: TestVector ; $ crate :: dev :: blobby :: parse_into_structs ! (include_bytes ! (concat ! ("data/" , stringify ! ($ name) , ".blb")) ; static TEST_VECTORS : & [TestVector { input , output }] ;) ; for (i , tv) in TEST_VECTORS . iter () . enumerate () { if let Err (reason) = $ test_fn ::<$ hasher > (tv) { panic ! ("\n\
                        Failed test #{i}:\n\
                        reason:\t{reason}\n\
                        test vector:\t{tv:?}\n") ; } } } } ; }
};
}
