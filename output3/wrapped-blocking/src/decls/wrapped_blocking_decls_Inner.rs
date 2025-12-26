use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Inner state of the blocking executor.
struct Inner {
    /// Number of idle threads in the pool.
    ///
    /// Idle threads are sleeping, waiting to get a task to run.
    idle_count: usize,
    /// Total number of threads in the pool.
    ///
    /// This is the number of idle threads + the number of active threads.
    thread_count: usize,
    /// The queue of blocking tasks.
    queue: VecDeque<Runnable>,
    /// Maximum number of threads in the pool
    thread_limit: Option<NonZeroUsize>,
}
