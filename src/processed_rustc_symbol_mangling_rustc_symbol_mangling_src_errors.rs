// SRC: ../rust/compiler/rustc_symbol_mangling/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
// Errors emitted by symbol_mangling.

use std::fmt;

use crate::rustc_complete::{Diag, DiagCtxtHandle, Diagnostic, EmissionGuarantee, Level};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=TestOutput | COMPLEXITY=2 | LINES=7 */
use crate::rustc_complete::Span;

pub struct TestOutput {
    pub span: Span,
    pub kind: Kind,
    pub content: String,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=9 | LINES=12 */

// This diagnostic doesn't need translation because (a) it doesn't contain any
// natural language, and (b) it's only used in tests. So we construct it
// manually and avoid the fluent machinery.
impl<G: EmissionGuarantee> Diagnostic<'_, G> for TestOutput {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let TestOutput { span, kind, content } = self;

        #[allow(rustc::untranslatable_diagnostic)]
        Diag::new(dcx, level, format!("{kind}({content})")).with_span(span)
    }
}
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

pub enum Kind {
    SymbolName,
    Demangling,
    DemanglingAlt,
    DefPath,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=11 */

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::SymbolName => write!(f, "symbol-name"),
            Kind::Demangling => write!(f, "demangling"),
            Kind::DemanglingAlt => write!(f, "demangling-alt"),
            Kind::DefPath => write!(f, "def-path"),
        }
    }
}