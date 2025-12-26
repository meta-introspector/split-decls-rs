use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Sink for demangled text that reports syntactic structure.
pub trait DemangleWrite {
    /// Called when we are entering the scope of some AST node.
    fn push_demangle_node(&mut self, _: DemangleNodeType) {}
    /// Same as `fmt::Write::write_str`.
    fn write_string(&mut self, s: &str) -> fmt::Result;
    /// Called when we are exiting the scope of some AST node for
    /// which `push_demangle_node` was called.
    fn pop_demangle_node(&mut self) {}
}
