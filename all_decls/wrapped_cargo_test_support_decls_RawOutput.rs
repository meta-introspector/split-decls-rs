use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This is the raw output from the process.
///
/// This is similar to `std::process::Output`, however the `status` is
/// translated to the raw `code`. This is necessary because `ProcessError`
/// does not have access to the raw `ExitStatus` because `ProcessError` needs
/// to be serializable (for the Rustc cache), and `ExitStatus` does not
/// provide a constructor.
pub struct RawOutput {
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
