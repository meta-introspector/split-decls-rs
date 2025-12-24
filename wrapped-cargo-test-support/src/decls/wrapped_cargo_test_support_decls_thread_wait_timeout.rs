use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper that waits for a thread to finish, up to `n` tenths of a second.
#[track_caller]
pub fn thread_wait_timeout<T>(n: u32, thread: JoinHandle<T>) -> T {
    retry(n, || thread.is_finished().then_some(()));
    thread.join().unwrap()
}
