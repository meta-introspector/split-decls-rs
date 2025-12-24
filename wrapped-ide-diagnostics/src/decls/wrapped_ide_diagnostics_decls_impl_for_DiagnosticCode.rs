use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DiagnosticCode {
    pub fn url(&self) -> String {
        match self {
            DiagnosticCode::RustcHardError(e) => {
                format!("https://doc.rust-lang.org/stable/error_codes/{e}.html")
            }
            DiagnosticCode::SyntaxError => {
                String::from("https://doc.rust-lang.org/stable/reference/")
            }
            DiagnosticCode::RustcLint(e) => {
                format!("https://doc.rust-lang.org/rustc/?search={e}")
            }
            DiagnosticCode::Clippy(e) => {
                format!("https://rust-lang.github.io/rust-clippy/master/#/{e}")
            }
            DiagnosticCode::Ra(e, _) => {
                format!("https://rust-analyzer.github.io/book/diagnostics.html#{e}")
            }
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCode::RustcHardError(r)
            | DiagnosticCode::RustcLint(r)
            | DiagnosticCode::Clippy(r)
            | DiagnosticCode::Ra(r, _) => r,
            DiagnosticCode::SyntaxError => "syntax-error",
        }
    }
}
