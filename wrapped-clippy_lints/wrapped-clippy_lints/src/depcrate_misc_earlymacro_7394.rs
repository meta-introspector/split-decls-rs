// Generated macro for macro_7394 (macro)
macro_rules! Depcrate_misc_earlymacro_7394 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7394"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for tuple patterns with a wildcard"] # [doc = " pattern (`_`) is next to a rest pattern (`..`)."] # [doc = ""] # [doc = " _NOTE_: While `_, ..` means there is at least one element left, `..`"] # [doc = " means there are 0 or more elements left. This can make a difference"] # [doc = " when refactoring, but shouldn't result in errors in the refactored code,"] # [doc = " since the wildcard pattern isn't used anyway."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The wildcard pattern is unneeded as the rest pattern"] # [doc = " can match that element as well."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct TupleStruct(u32, u32, u32);"] # [doc = " # let t = TupleStruct(1, 2, 3);"] # [doc = " match t {"] # [doc = "     TupleStruct(0, .., _) => (),"] # [doc = "     _ => (),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct TupleStruct(u32, u32, u32);"] # [doc = " # let t = TupleStruct(1, 2, 3);"] # [doc = " match t {"] # [doc = "     TupleStruct(0, ..) => (),"] # [doc = "     _ => (),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub UNNEEDED_WILDCARD_PATTERN , complexity , "tuple patterns with a wildcard pattern (`_`) is next to a rest pattern (`..`)" }
};
}
