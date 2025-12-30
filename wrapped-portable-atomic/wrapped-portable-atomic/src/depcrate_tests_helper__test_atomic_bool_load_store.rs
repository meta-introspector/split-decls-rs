// Generated macro for __test_atomic_bool_load_store (macro)
macro_rules! Depcrate_tests_helper__test_atomic_bool_load_store {
() => {
// Module: crate::tests::helper
// Provides: {"__test_atomic_bool_load_store"}
// Dependencies: {}
macro_rules ! __test_atomic_bool_load_store { ($ atomic_type : ty , single_thread) => { __test_atomic_common ! ($ atomic_type , bool) ; use crate :: tests :: helper :: { self , * } ; # [test] fn accessor () { let a = <$ atomic_type >:: new (false) ; unsafe { assert_eq ! (* a . as_ptr () , false) ; * a . as_ptr () = true ; assert_eq ! (a . as_ptr () as * const () , & a as * const _ as * const ()) ; assert_eq ! (* a . as_ptr () , true) ; } } # [test] fn static_load_only () { static VAR : $ atomic_type = <$ atomic_type >:: new (false) ; for & order in & helper :: LOAD_ORDERINGS { assert_eq ! (VAR . load (order) , false) ; } } # [test] fn load_store () { static VAR : $ atomic_type = <$ atomic_type >:: new (false) ; test_load_ordering (| order | VAR . load (order)) ; test_store_ordering (| order | VAR . store (false , order)) ; for (& load_order , & store_order) in helper :: LOAD_ORDERINGS . iter () . zip (& helper :: STORE_ORDERINGS) { assert_eq ! (VAR . load (load_order) , false) ; VAR . store (true , store_order) ; assert_eq ! (VAR . load (load_order) , true) ; VAR . store (false , store_order) ; let a = <$ atomic_type >:: new (true) ; assert_eq ! (a . load (load_order) , true) ; a . store (false , store_order) ; assert_eq ! (a . load (load_order) , false) ; } } } ; ($ atomic_type : ty) => { __test_atomic_bool_load_store ! ($ atomic_type , single_thread) ; } ; }
};
}
