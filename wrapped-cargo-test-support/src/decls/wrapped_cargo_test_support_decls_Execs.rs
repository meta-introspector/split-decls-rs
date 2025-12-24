use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Run and verify a [`ProcessBuilder`]
///
/// Construct with
/// - [`execs`]
/// - [`Project`] methods
/// - `cargo_process` in testsuite
#[must_use]
#[derive(Clone)]
pub struct Execs {
    ran: bool,
    process_builder: Option<ProcessBuilder>,
    expect_stdin: Option<String>,
    expect_exit_code: Option<i32>,
    expect_stdout_data: Option<snapbox::Data>,
    expect_stderr_data: Option<snapbox::Data>,
    expect_stdout_contains: Vec<String>,
    expect_stderr_contains: Vec<String>,
    expect_stdout_not_contains: Vec<String>,
    expect_stderr_not_contains: Vec<String>,
    expect_stderr_with_without: Vec<(Vec<String>, Vec<String>)>,
    stream_output: bool,
    assert: snapbox::Assert,
}
