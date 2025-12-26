use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The thread identifier of the calling thread.
pub fn thread_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}
