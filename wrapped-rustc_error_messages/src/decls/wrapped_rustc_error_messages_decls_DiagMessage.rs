use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Abstraction over a message in a diagnostic to support both translatable and non-translatable
/// diagnostic messages.
///
/// Intended to be removed once diagnostics are entirely translatable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Encodable, Decodable)]
#[rustc_diagnostic_item = "DiagMessage"]
pub enum DiagMessage {
    /// Non-translatable diagnostic message.
    Str(Cow<'static, str>),
    /// Translatable message which has been already translated.
    ///
    /// Some diagnostics have repeated subdiagnostics where the same interpolated variables would
    /// be instantiated multiple times with different values. These subdiagnostics' messages
    /// are translated when they are added to the parent diagnostic, producing this variant of
    /// `DiagMessage`.
    Translated(Cow<'static, str>),
    /// Identifier for a Fluent message (with optional attribute) corresponding to the diagnostic
    /// message. Yet to be translated.
    ///
    /// <https://projectfluent.org/fluent/guide/hello.html>
    /// <https://projectfluent.org/fluent/guide/attributes.html>
    FluentIdentifier(FluentId, Option<FluentId>),
}
