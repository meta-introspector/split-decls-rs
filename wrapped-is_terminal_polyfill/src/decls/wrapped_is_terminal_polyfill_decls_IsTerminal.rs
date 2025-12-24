use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Trait to determine if a descriptor/handle refers to a terminal/tty.
pub trait IsTerminal: sealed::Sealed {
    /// Returns `true` if the descriptor/handle refers to a terminal/tty.
    ///
    /// On platforms where Rust does not know how to detect a terminal yet, this will return
    /// `false`. This will also return `false` if an unexpected error occurred, such as from
    /// passing an invalid file descriptor.
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows, in addition to detecting consoles, this currently uses some heuristics to
    /// detect older msys/cygwin/mingw pseudo-terminals based on device name: devices with names
    /// starting with `msys-` or `cygwin-` and ending in `-pty` will be considered terminals.
    /// Note that this [may change in the future][changes].
    ///
    /// [changes]: std::io#platform-specific-behavior
    fn is_terminal(&self) -> bool;
}
