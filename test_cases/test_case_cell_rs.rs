// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/cell.rs
// Error: expected square brackets
// Problematic line: line 265

mod lazy;
mod once;

#[stable(feature = "lazy_cell", since = "1.80.0")]
pub use lazy::LazyCell;
#[stable(feature = "once_cell", since = "1.70.0")]
pub use once::OnceCell;
