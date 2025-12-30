// Generated macro for new_fail_test (macro)
macro_rules! Depcrate_devnew_fail_test {
() => {
// Module: crate::dev
// Provides: {"new_fail_test"}
// Dependencies: {}
# [doc = " Define AEAD test for failing test vectors"] # [macro_export] macro_rules ! new_fail_test { ($ name : ident , $ test_name : expr , $ cipher : ty $ (,) ?) => { # [test] fn $ name () { use $ crate :: dev :: TestVector ; $ crate :: dev :: blobby :: parse_into_structs ! (include_bytes ! (concat ! ("data/" , $ test_name , ".blb")) ; static TEST_VECTORS : & [TestVector { key , nonce , aad , plaintext , ciphertext }] ;) ; for (i , tv) in TEST_VECTORS . iter () . enumerate () { let res = $ crate :: dev :: fail_test ::<$ cipher > (tv) ; if let Err (reason) = res { panic ! ("\n\
                        Failed test #{i}\n\
                        reason:\t{reason:?}\n\
                        test vector:\t{tv:?}\n") ; } } } } ; }
};
}
