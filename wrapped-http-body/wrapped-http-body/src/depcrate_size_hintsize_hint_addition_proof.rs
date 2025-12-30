// Generated macro for size_hint_addition_proof (function)
macro_rules! Depcrate_size_hintsize_hint_addition_proof {
() => {
// Module: crate::size_hint
// Provides: {"size_hint_addition_proof"}
// Dependencies: {}
# [doc = " Asserts that SizeHint addition is perfect with a basic proof"] # [test] fn size_hint_addition_proof () { # [doc = " Converts a SizeHint to a tuple for equality checks and matching"] fn to_parts (s : SizeHint) -> (u64 , Option < u64 >) { (s . lower () , s . upper ()) } match (to_parts (SizeHint :: new ()) , to_parts (SizeHint :: new ())) { ((_ , Some (_)) , (_ , Some (_))) => { } ((_ , None) , (_ , None)) => { } ((_ , Some (_)) , (_ , None)) => { } ((_ , None) , (_ , Some (_))) => { } } macro_rules ! reciprocal_add_eq { ($ a : expr , $ b : expr , $ eq : expr) => { assert_eq ! (to_parts (($ a . clone () + $ b . clone ())) , $ eq) ; assert_eq ! (to_parts (($ b . clone () + $ a . clone ())) , $ eq) ; } ; } let exact_1 = SizeHint :: with_exact (1) ; let exact_2 = SizeHint :: with_exact (2) ; reciprocal_add_eq ! (exact_1 , exact_2 , to_parts (SizeHint :: with_exact (1 + 2))) ; let some_lhs = SizeHint { lower : 4 , upper : Some (8) , } ; let some_rhs = SizeHint { lower : 16 , upper : Some (32) , } ; reciprocal_add_eq ! (some_lhs , some_rhs , (4 + 16 , Some (8 + 32))) ; let none_lhs = SizeHint { lower : 64 , upper : None , } ; let none_rhs = SizeHint { lower : 128 , upper : None , } ; reciprocal_add_eq ! (none_lhs , none_rhs , (64 + 128 , None)) ; reciprocal_add_eq ! (some_lhs , none_rhs , (4 + 128 , None)) ; }
};
}
