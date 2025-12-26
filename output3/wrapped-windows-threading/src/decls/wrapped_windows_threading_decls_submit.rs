use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Submit the closure to the default thread pool.
///
/// * The closure must have `'static` lifetime as the thread may outlive the lifetime in which `submit` is called.
/// * The closure must be `Send` as it will be sent to another thread for execution.
pub fn submit<F: FnOnce() + Send + 'static>(f: F) {
    unsafe {
        try_submit(core::ptr::null(), f);
    }
}
