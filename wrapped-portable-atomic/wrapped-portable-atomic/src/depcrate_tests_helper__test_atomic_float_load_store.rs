// Generated macro for __test_atomic_float_load_store (macro)
macro_rules! Depcrate_tests_helper__test_atomic_float_load_store {
() => {
// Module: crate::tests::helper
// Provides: {"__test_atomic_float_load_store"}
// Dependencies: {}
macro_rules ! __test_atomic_float_load_store { ($ atomic_type : ty , $ float_type : ident , single_thread) => { __test_atomic_common ! ($ atomic_type , $ float_type) ; use crate :: tests :: helper :: { self , * } ; # [test] fn accessor () { let a = <$ atomic_type >:: new (10.) ; unsafe { assert_eq ! (* a . as_ptr () , 10.) ; * a . as_ptr () = 5. ; assert_eq ! (a . as_ptr () as * const () , & a as * const _ as * const ()) ; assert_eq ! (* a . as_ptr () , 5.) ; } } # [test] fn static_load_only () { static VAR : $ atomic_type = <$ atomic_type >:: new (10.) ; for & order in & helper :: LOAD_ORDERINGS { assert_eq ! (VAR . load (order) , 10.) ; } } # [test] fn load_store () { static VAR : $ atomic_type = <$ atomic_type >:: new (10.) ; test_load_ordering (| order | VAR . load (order)) ; test_store_ordering (| order | VAR . store (10. , order)) ; for (& load_order , & store_order) in helper :: LOAD_ORDERINGS . iter () . zip (& helper :: STORE_ORDERINGS) { assert_eq ! (VAR . load (load_order) , 10.) ; VAR . store (5. , store_order) ; assert_eq ! (VAR . load (load_order) , 5.) ; VAR . store (10. , store_order) ; let a = <$ atomic_type >:: new (1.) ; assert_eq ! (a . load (load_order) , 1.) ; a . store (2. , store_order) ; assert_eq ! (a . load (load_order) , 2.) ; } } } ; ($ atomic_type : ty , $ float_type : ident) => { __test_atomic_float_load_store ! ($ atomic_type , $ float_type , single_thread) ; } ; }
};
}
