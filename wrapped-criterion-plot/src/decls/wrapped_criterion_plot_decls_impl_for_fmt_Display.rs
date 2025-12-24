use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::Exec(err) => write!(f, "`gnuplot --version` failed: {}", err),
            VersionError::Error(msg) => {
                write!(f, "`gnuplot --version` failed with error message:\n{}", msg)
            }
            VersionError::OutputError => {
                write!(f, "`gnuplot --version` returned invalid utf-8")
            }
            VersionError::ParseError(msg) => {
                write!(
                    f, "`gnuplot --version` returned an unparsable version string: {}",
                    msg
                )
            }
        }
    }
}
