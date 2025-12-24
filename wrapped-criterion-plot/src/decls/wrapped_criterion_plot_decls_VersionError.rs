use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Possible errors when parsing gnuplot's version string
#[derive(Debug)]
pub enum VersionError {
    /// The `gnuplot` command couldn't be executed
    Exec(io::Error),
    /// The `gnuplot` command returned an error message
    Error(String),
    /// The `gnuplot` command returned invalid utf-8
    OutputError,
    /// The `gnuplot` command returned an unparsable string
    ParseError(String),
}
