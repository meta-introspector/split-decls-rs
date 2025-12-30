// Generated macro for macro_7095 (macro)
macro_rules! Depcrate_methodsmacro_7095 {
() => {
// Module: crate::methods
// Provides: {"macro_7095"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for `Iterator::last` being called on a  `DoubleEndedIterator`, which can be replaced"] # [doc = " with `DoubleEndedIterator::next_back`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " `Iterator::last` is implemented by consuming the iterator, which is unnecessary if"] # [doc = " the iterator is a `DoubleEndedIterator`. Since Rust traits do not allow specialization,"] # [doc = " `Iterator::last` cannot be optimized for `DoubleEndedIterator`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let last_arg = \"echo hello world\".split(' ').last();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let last_arg = \"echo hello world\".split(' ').next_back();"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub DOUBLE_ENDED_ITERATOR_LAST , perf , "using `Iterator::last` on a `DoubleEndedIterator`" }
};
}
