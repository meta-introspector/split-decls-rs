use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A handle to an external process which load dylibs with macros (.so or .dll)
/// and runs actual macro expansion functions.
#[derive(Debug)]
pub struct ProcMacroClient {
    /// Currently, the proc macro process expands all procedural macros sequentially.
    ///
    /// That means that concurrent salsa requests may block each other when expanding proc macros,
    /// which is unfortunate, but simple and good enough for the time being.
    process: Arc<ProcMacroServerProcess>,
    path: AbsPathBuf,
}
