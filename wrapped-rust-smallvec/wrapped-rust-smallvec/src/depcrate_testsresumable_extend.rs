// Generated macro for resumable_extend (function)
macro_rules! Depcrate_testsresumable_extend {
() => {
// Module: crate::tests
// Provides: {"resumable_extend"}
// Dependencies: {}
# [test] fn resumable_extend () { let s = "a b c" ; let it = s . chars () . scan (0 , | _ , ch | if ch . is_whitespace () { None } else { Some (ch) }) ; let mut v : SmallVec < char , 4 > = SmallVec :: new () ; v . extend (it) ; assert_eq ! (v [..] , ['a']) ; }
};
}
