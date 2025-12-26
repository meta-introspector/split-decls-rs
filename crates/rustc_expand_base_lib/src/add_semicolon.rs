//! This module defines the `AddSemicolon` enum, which is a simple utility used during macro
//! expansion to indicate whether a generated statement requires a trailing semicolon.
//!
//! For n00bs: In Rust, some statements (like variable declarations) need a semicolon at the end,
//! while expressions (like `5 + 3`) usually don't. When macros generate code, they sometimes
//! need to decide whether to add this semicolon. This enum helps make that decision clear.
//!
//! For example, if a macro expands to `let x = 10`, it usually needs to become `let x = 10;`.
//! This enum helps track whether the generated code needs that final `;`.

/// Indicates whether a generated statement needs a trailing semicolon.
///
/// This enum is used in the context of macro expansion, particularly when generating
/// statements, to manage the presence or absence of a trailing semicolon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddSemicolon {
    /// The generated statement requires a semicolon.
    Yes,
    /// The generated statement does not require a semicolon.
    No,
}
