// Generated macro for assert_size_hint (function)
macro_rules! Depcrate_testassert_size_hint {
() => {
// Module: crate::test
// Provides: {"assert_size_hint"}
// Dependencies: {}
fn assert_size_hint < T > (arena_len : usize , iter : IterMut < '_ , T >) { let (min , max) = iter . size_hint () ; assert ! (max . is_some ()) ; let max = max . unwrap () ; assert ! (min <= arena_len) ; assert ! (max >= arena_len) ; assert ! (min >= arena_len / 3) ; assert ! (max <= arena_len * 3) ; }
};
}
