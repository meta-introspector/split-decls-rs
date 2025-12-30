// Generated macro for macro_4000 (macro)
macro_rules! Depcrate_manual_assertmacro_4000 {
() => {
// Module: crate::manual_assert
// Provides: {"macro_4000"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects `if`-then-`panic!` that can be replaced with `assert!`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `assert!` is simpler than `if`-then-`panic!`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let sad_people: Vec<&str> = vec![];"] # [doc = " if !sad_people.is_empty() {"] # [doc = "     panic!(\"there are sad people: {:?}\", sad_people);"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let sad_people: Vec<&str> = vec![];"] # [doc = " assert!(sad_people.is_empty(), \"there are sad people: {:?}\", sad_people);"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub MANUAL_ASSERT , pedantic , "`panic!` and only a `panic!` in `if`-then statement" }
};
}
