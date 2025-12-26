use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// | Level        | is_error | EmissionGuarantee            | Top-level | Sub | Used in lints?
/// | -----        | -------- | -----------------            | --------- | --- | --------------
/// | Bug          | yes      | BugAbort                     | yes       | -   | -
/// | Fatal        | yes      | FatalAbort/FatalError[^star] | yes       | -   | -
/// | Error        | yes      | ErrorGuaranteed              | yes       | -   | yes
/// | DelayedBug   | yes      | ErrorGuaranteed              | yes       | -   | -
/// | ForceWarning | -        | ()                           | yes       | -   | lint-only
/// | Warning      | -        | ()                           | yes       | yes | yes
/// | Note         | -        | ()                           | rare      | yes | -
/// | OnceNote     | -        | ()                           | -         | yes | lint-only
/// | Help         | -        | ()                           | rare      | yes | -
/// | OnceHelp     | -        | ()                           | -         | yes | lint-only
/// | FailureNote  | -        | ()                           | rare      | -   | -
/// | Allow        | -        | ()                           | yes       | -   | lint-only
/// | Expect       | -        | ()                           | yes       | -   | lint-only
///
/// [^star]: `FatalAbort` normally, `FatalError` in the non-aborting "almost fatal" case that is
///     occasionally used.
///
#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug, Encodable, Decodable)]
pub enum Level {
    /// For bugs in the compiler. Manifests as an ICE (internal compiler error) panic.
    Bug,
    /// An error that causes an immediate abort. Used for things like configuration errors,
    /// internal overflows, some file operation errors.
    Fatal,
    /// An error in the code being compiled, which prevents compilation from finishing. This is the
    /// most common case.
    Error,
    /// This is a strange one: lets you register an error without emitting it. If compilation ends
    /// without any other errors occurring, this will be emitted as a bug. Otherwise, it will be
    /// silently dropped. I.e. "expect other errors are emitted" semantics. Useful on code paths
    /// that should only be reached when compiling erroneous code.
    DelayedBug,
    /// A `force-warn` lint warning about the code being compiled. Does not prevent compilation
    /// from finishing.
    ///
    /// Requires a [`LintExpectationId`] for expected lint diagnostics. In all other cases this
    /// should be `None`.
    ForceWarning,
    /// A warning about the code being compiled. Does not prevent compilation from finishing.
    /// Will be skipped if `can_emit_warnings` is false.
    Warning,
    /// A message giving additional context.
    Note,
    /// A note that is only emitted once.
    OnceNote,
    /// A message suggesting how to fix something.
    Help,
    /// A help that is only emitted once.
    OnceHelp,
    /// Similar to `Note`, but used in cases where compilation has failed. When printed for human
    /// consumption, it doesn't have any kind of `note:` label.
    FailureNote,
    /// Only used for lints.
    Allow,
    /// Only used for lints. Requires a [`LintExpectationId`] for silencing the lints.
    Expect,
}
