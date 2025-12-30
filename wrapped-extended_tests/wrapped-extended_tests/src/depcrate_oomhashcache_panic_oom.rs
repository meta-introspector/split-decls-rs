// Generated macro for hashcache_panic_oom (function)
macro_rules! Depcrate_oomhashcache_panic_oom {
() => {
// Module: crate::oom
// Provides: {"hashcache_panic_oom"}
// Dependencies: {}
fn hashcache_panic_oom (repeat : usize) { let hashcache : HashCache < usize , R > = HashCache :: default () ; for k in 0 .. repeat { let result : Result < () , Box < dyn Any + Send > > = test_oom (| | { assert ! (hashcache . put_sync (k , R :: new (& INST_CNT , true)) . is_ok ()) ; }) ; assert_eq ! (hashcache . get_sync (& k) . is_some () , result . is_ok ()) ; } drop (hashcache) ; }
};
}
