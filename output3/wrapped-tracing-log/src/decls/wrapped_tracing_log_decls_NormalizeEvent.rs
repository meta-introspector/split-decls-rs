use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Extends log `Event`s to provide complete `Metadata`.
///
/// In `tracing-log`, an `Event` produced by a log (through [`AsTrace`]) has an hard coded
/// "log" target and no `file`, `line`, or `module_path` attributes. This happens because `Event`
/// requires its `Metadata` to be `'static`, while [`log::Record`]s provide them with a generic
/// lifetime.
///
/// However, these values are stored in the `Event`'s fields and
/// the [`normalized_metadata`] method allows to build a new `Metadata`
/// that only lives as long as its source `Event`, but provides complete
/// data.
///
/// It can typically be used by `Subscriber`s when processing an `Event`,
/// to allow accessing its complete metadata in a consistent way,
/// regardless of the source of its source.
///
/// [`normalized_metadata`]: NormalizeEvent#normalized_metadata
pub trait NormalizeEvent<'a>: crate::sealed::Sealed {
    /// If this `Event` comes from a `log`, this method provides a new
    /// normalized `Metadata` which has all available attributes
    /// from the original log, including `file`, `line`, `module_path`
    /// and `target`.
    /// Returns `None` is the `Event` is not issued from a `log`.
    fn normalized_metadata(&'a self) -> Option<Metadata<'a>>;
    /// Returns whether this `Event` represents a log (from the `log` crate)
    fn is_log(&self) -> bool;
}
