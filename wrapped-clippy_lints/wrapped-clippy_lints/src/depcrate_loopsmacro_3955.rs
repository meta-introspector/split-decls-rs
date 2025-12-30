// Generated macro for macro_3955 (macro)
macro_rules! Depcrate_loopsmacro_3955 {
() => {
// Module: crate::loops
// Provides: {"macro_3955"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks whether a for loop is being used to push a constant"] # [doc = " value into a Vec."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This kind of operation can be expressed more succinctly with"] # [doc = " `vec![item; SIZE]` or `vec.resize(NEW_SIZE, item)` and using these alternatives may also"] # [doc = " have better performance."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let item1 = 2;"] # [doc = " let item2 = 3;"] # [doc = " let mut vec: Vec<u8> = Vec::new();"] # [doc = " for _ in 0..20 {"] # [doc = "     vec.push(item1);"] # [doc = " }"] # [doc = " for _ in 0..30 {"] # [doc = "     vec.push(item2);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let item1 = 2;"] # [doc = " let item2 = 3;"] # [doc = " let mut vec: Vec<u8> = vec![item1; 20];"] # [doc = " vec.resize(20 + 30, item2);"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub SAME_ITEM_PUSH , style , "the same item is pushed inside of a for loop" }
};
}
