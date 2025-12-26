use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The reason for future incompatibility
///
/// Future-incompatible lints come in roughly two categories:
///
/// 1. There was a mistake in the compiler (such as a soundness issue), and
///    we're trying to fix it, but it may be a breaking change.
/// 2. A change across an Edition boundary, typically used for the
///    introduction of new language features that can't otherwise be
///    introduced in a backwards-compatible way.
///
/// See <https://rustc-dev-guide.rust-lang.org/bug-fix-procedure.html> and
/// <https://rustc-dev-guide.rust-lang.org/diagnostics.html#future-incompatible-lints>
/// for more information.
#[derive(Copy, Clone, Debug)]
pub enum FutureIncompatibilityReason {
    /// This will be an error in a future release for all editions
    ///
    /// Choose this variant when you are first introducing a "future
    /// incompatible" warning that is intended to eventually be fixed in the
    /// future.
    ///
    /// After a lint has been in this state for a while and you feel like it is ready to graduate
    /// to warning everyone, consider setting [`FutureIncompatibleInfo::report_in_deps`] to true.
    /// (see its documentation for more guidance)
    ///
    /// After some period of time, lints with this variant can be turned into
    /// hard errors (and the lint removed). Preferably when there is some
    /// confidence that the number of impacted projects is very small (few
    /// should have a broken dependency in their dependency tree).
    FutureReleaseError,
    /// Code that changes meaning in some way in a
    /// future release.
    ///
    /// Choose this variant when the semantics of existing code is changing,
    /// (as opposed to [`FutureIncompatibilityReason::FutureReleaseError`],
    /// which is for when code is going to be rejected in the future).
    FutureReleaseSemanticsChange,
    /// Previously accepted code that will become an
    /// error in the provided edition
    ///
    /// Choose this variant for code that you want to start rejecting across
    /// an edition boundary. This will automatically include the lint in the
    /// `rust-20xx-compatibility` lint group, which is used by `cargo fix
    /// --edition` to do migrations. The lint *should* be auto-fixable with
    /// [`Applicability::MachineApplicable`].
    ///
    /// The lint can either be `Allow` or `Warn` by default. If it is `Allow`,
    /// users usually won't see this warning unless they are doing an edition
    /// migration manually or there is a problem during the migration (cargo's
    /// automatic migrations will force the level to `Warn`). If it is `Warn`
    /// by default, users on all editions will see this warning (only do this
    /// if you think it is important for everyone to be aware of the change,
    /// and to encourage people to update their code on all editions).
    ///
    /// See also [`FutureIncompatibilityReason::EditionSemanticsChange`] if
    /// you have code that is changing semantics across the edition (as
    /// opposed to being rejected).
    EditionError(Edition),
    /// Code that changes meaning in some way in
    /// the provided edition
    ///
    /// This is the same as [`FutureIncompatibilityReason::EditionError`],
    /// except for situations where the semantics change across an edition. It
    /// slightly changes the text of the diagnostic, but is otherwise the
    /// same.
    EditionSemanticsChange(Edition),
    /// This will be an error in the provided edition *and* in a future
    /// release.
    ///
    /// This variant a combination of [`FutureReleaseError`] and [`EditionError`].
    /// This is useful in rare cases when we want to have "preview" of a breaking
    /// change in an edition, but do a breaking change later on all editions anyway.
    ///
    /// [`EditionError`]: FutureIncompatibilityReason::EditionError
    /// [`FutureReleaseError`]: FutureIncompatibilityReason::FutureReleaseError
    EditionAndFutureReleaseError(Edition),
    /// This will change meaning in the provided edition *and* in a future
    /// release.
    ///
    /// This variant a combination of [`FutureReleaseSemanticsChange`]
    /// and [`EditionSemanticsChange`]. This is useful in rare cases when we
    /// want to have "preview" of a breaking change in an edition, but do a
    /// breaking change later on all editions anyway.
    ///
    /// [`EditionSemanticsChange`]: FutureIncompatibilityReason::EditionSemanticsChange
    /// [`FutureReleaseSemanticsChange`]: FutureIncompatibilityReason::FutureReleaseSemanticsChange
    EditionAndFutureReleaseSemanticsChange(Edition),
    /// A custom reason.
    ///
    /// Choose this variant if the built-in text of the diagnostic of the
    /// other variants doesn't match your situation. This is behaviorally
    /// equivalent to
    /// [`FutureIncompatibilityReason::FutureReleaseError`].
    Custom(&'static str),
}
