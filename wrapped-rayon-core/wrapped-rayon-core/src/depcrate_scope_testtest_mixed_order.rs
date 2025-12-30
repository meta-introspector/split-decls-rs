// Generated macro for test_mixed_order (macro)
macro_rules! Depcrate_scope_testtest_mixed_order {
() => {
// Module: crate::scope::test
// Provides: {"test_mixed_order"}
// Dependencies: {}
# [doc = " Test spawns pushing a series of numbers, interleaved"] # [doc = " such that negative values are using an inner scope."] macro_rules ! test_mixed_order { ($ outer_scope : ident => $ outer_spawn : ident , $ inner_scope : ident => $ inner_spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; $ outer_scope (| outer_scope | { let vec = & vec ; spawn_push ! (outer_scope .$ outer_spawn , vec , 0) ; $ inner_scope (| inner_scope | { spawn_push ! (inner_scope .$ inner_spawn , vec , - 1) ; spawn_push ! (outer_scope .$ outer_spawn , vec , 1) ; spawn_push ! (inner_scope .$ inner_spawn , vec , - 2) ; spawn_push ! (outer_scope .$ outer_spawn , vec , 2) ; spawn_push ! (inner_scope .$ inner_spawn , vec , - 3) ; }) ; spawn_push ! (outer_scope .$ outer_spawn , vec , 3) ; }) ; vec . into_inner () . unwrap () }) } } ; }
};
}
