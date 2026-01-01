// SRC: ../rust/compiler/rustc_errors/src/diagnostic_impls.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use std::borrow::Cow;

use crate::rustc_abi::TargetDataLayoutErrors;
use crate::rustc_error_messages::{DiagArgValue, IntoDiagArg};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use rustc_macros::Subdiagnostic;
use crate::rustc_complete::{Span, Symbol};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

use crate::diagnostic::DiagLocation;
use crate::{
    Diag, DiagCtxtHandle, Diagnostic, EmissionGuarantee, Level, Subdiagnostic,
    fluent_generated as fluent,
};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6 */

impl IntoDiagArg for DiagLocation {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::from(self.to_string()))
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=DiagSymbolList | COMPLEXITY=5 | LINES=9 */

#[derive(Clone)]
pub struct DiagSymbolList<S = Symbol>(Vec<S>);

impl<S> From<Vec<S>> for DiagSymbolList<S> {
    fn from(v: Vec<S>) -> Self {
        DiagSymbolList(v)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=6 */

impl<S> FromIterator<S> for DiagSymbolList<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=6 | LINES=8 */

impl<S: std::fmt::Display> IntoDiagArg for DiagSymbolList<S> {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::StrListSepByAnd(
            self.0.into_iter().map(|sym| Cow::Owned(format!("`{sym}`"))).collect(),
        )
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=29 | LINES=47 */

impl<G: EmissionGuarantee> Diagnostic<'_, G> for TargetDataLayoutErrors<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        match self {
            TargetDataLayoutErrors::InvalidAddressSpace { addr_space, err, cause } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_address_space)
                    .with_arg("addr_space", addr_space)
                    .with_arg("cause", cause)
                    .with_arg("err", err)
            }
            TargetDataLayoutErrors::InvalidBits { kind, bit, cause, err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_bits)
                    .with_arg("kind", kind)
                    .with_arg("bit", bit)
                    .with_arg("cause", cause)
                    .with_arg("err", err)
            }
            TargetDataLayoutErrors::MissingAlignment { cause } => {
                Diag::new(dcx, level, fluent::errors_target_missing_alignment)
                    .with_arg("cause", cause)
            }
            TargetDataLayoutErrors::InvalidAlignment { cause, err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_alignment)
                    .with_arg("cause", cause)
                    .with_arg("err_kind", err.diag_ident())
                    .with_arg("align", err.align())
            }
            TargetDataLayoutErrors::InconsistentTargetArchitecture { dl, target } => {
                Diag::new(dcx, level, fluent::errors_target_inconsistent_architecture)
                    .with_arg("dl", dl)
                    .with_arg("target", target)
            }
            TargetDataLayoutErrors::InconsistentTargetPointerWidth { pointer_size, target } => {
                Diag::new(dcx, level, fluent::errors_target_inconsistent_pointer_width)
                    .with_arg("pointer_size", pointer_size)
                    .with_arg("target", target)
            }
            TargetDataLayoutErrors::InvalidBitsSize { err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_bits_size).with_arg("err", err)
            }
            TargetDataLayoutErrors::UnknownPointerSpecification { err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_datalayout_pointer_spec)
                    .with_arg("err", err)
            }
        }
    }
}
/* AST_META: AST_ID=9 | TYPE=STRUCT | NAME=SingleLabelManySpans | COMPLEXITY=4 | LINES=6 */

/// Utility struct used to apply a single label while highlighting multiple spans
pub struct SingleLabelManySpans {
    pub spans: Vec<Span>,
    pub label: &'static str,
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=5 | LINES=5 */
impl Subdiagnostic for SingleLabelManySpans {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        diag.span_labels(self.spans, self.label);
    }
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=ExpectedLifetimeParameter | COMPLEXITY=2 | LINES=8 */

#[derive(Subdiagnostic)]
#[label(errors_expected_lifetime_parameter)]
pub struct ExpectedLifetimeParameter {
    #[primary_span]
    pub span: Span,
    pub count: usize,
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=IndicateAnonymousLifetime | COMPLEXITY=3 | LINES=9 */

#[derive(Subdiagnostic)]
#[suggestion(errors_indicate_anonymous_lifetime, code = "{suggestion}", style = "verbose")]
pub struct IndicateAnonymousLifetime {
    #[primary_span]
    pub span: Span,
    pub count: usize,
    pub suggestion: String,
}
/* AST_META: AST_ID=13 | TYPE=STRUCT | NAME=ElidedLifetimeInPathSubdiag | COMPLEXITY=2 | LINES=8 */

#[derive(Subdiagnostic)]
pub struct ElidedLifetimeInPathSubdiag {
    #[subdiagnostic]
    pub expected: ExpectedLifetimeParameter,
    #[subdiagnostic]
    pub indicate: Option<IndicateAnonymousLifetime>,
}