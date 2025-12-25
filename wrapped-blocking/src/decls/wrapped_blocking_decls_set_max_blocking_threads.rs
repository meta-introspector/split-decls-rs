use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Set the maximum number of threads used by the backing thread pool.
///
/// # Example
///
/// ```no_run
/// use blocking::unblock;
/// use std::fs::{read_dir, File};
/// use std::io::prelude::*;
/// # use std::num::NonZeroUsize;
///
/// blocking::set_max_blocking_threads(NonZeroUsize::new(100).unwrap());
///
/// # fn test() -> std::io::Result<()> {
/// let mut files = Vec::new();
/// for entry in read_dir("/path/to/large/directory").unwrap() {
///     files.push(unblock(move || -> std::io::Result<String> {
///         let mut contents = String::new();
///         let mut file = File::open(entry?.path())?;
///         file.read_to_string(&mut contents)?;
///         Ok(contents)
///     }));
/// }
/// # Ok(())
/// # }
/// ```
pub fn set_max_blocking_threads(threads: NonZeroUsize) {
    let executor = Executor::get();
    let mut inner = executor
        .inner
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let old_limit = inner.thread_limit;
    inner.thread_limit = Some(threads);
    if let Some(old_limit) = old_limit {
        if old_limit > threads {
            executor.cvar.notify_all();
        }
    }
}
