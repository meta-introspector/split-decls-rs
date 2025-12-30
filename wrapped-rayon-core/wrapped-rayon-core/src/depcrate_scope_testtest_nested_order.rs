// Generated macro for test_nested_order (macro)
macro_rules! Depcrate_scope_testtest_nested_order {
() => {
// Module: crate::scope::test
// Provides: {"test_nested_order"}
// Dependencies: {}
macro_rules ! test_nested_order { ($ outer_scope : ident => $ outer_spawn : ident , $ inner_scope : ident => $ inner_spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; $ outer_scope (| scope | { let vec = & vec ; for i in 0 .. 10 { scope .$ outer_spawn (move | _ | { $ inner_scope (| scope | { for j in 0 .. 10 { scope .$ inner_spawn (move | _ | { vec . lock () . unwrap () . push (i * 10 + j) ; }) ; } }) ; }) ; } }) ; vec . into_inner () . unwrap () }) } } ; }
};
}
