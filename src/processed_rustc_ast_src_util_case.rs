// SRC: ../rust/compiler/rustc_ast/src/util/case.rs
/* AST_META: AST_ID=1 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=6 */
/// Whatever to ignore case (`fn` vs `Fn` vs `FN`) or not. Used for recovering.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Case {
    Sensitive,
    Insensitive,
}