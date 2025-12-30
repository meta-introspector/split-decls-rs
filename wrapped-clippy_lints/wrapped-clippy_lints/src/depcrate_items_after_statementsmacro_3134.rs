// Generated macro for macro_3134 (macro)
macro_rules! Depcrate_items_after_statementsmacro_3134 {
() => {
// Module: crate::items_after_statements
// Provides: {"macro_3134"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items declared after some statement in a block."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Items live for the entire scope they are declared"] # [doc = " in. But statements are processed in order. This might cause confusion as"] # [doc = " it's hard to figure out which item is meant in a statement."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo() {"] # [doc = "     println!(\"cake\");"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     foo(); // prints \"foo\""] # [doc = "     fn foo() {"] # [doc = "         println!(\"foo\");"] # [doc = "     }"] # [doc = "     foo(); // prints \"foo\""] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo() {"] # [doc = "     println!(\"cake\");"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     fn foo() {"] # [doc = "         println!(\"foo\");"] # [doc = "     }"] # [doc = "     foo(); // prints \"foo\""] # [doc = "     foo(); // prints \"foo\""] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITEMS_AFTER_STATEMENTS , pedantic , "blocks where an item comes after a statement" }
};
}
