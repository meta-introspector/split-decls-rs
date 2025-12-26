use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns `gnuplot` version
pub fn version() -> Result<Version, VersionError> {
    let command_output = Command::new("gnuplot")
        .arg("--version")
        .output()
        .map_err(VersionError::Exec)?;
    if !command_output.status.success() {
        let error =
            String::from_utf8(command_output.stderr).map_err(|_| VersionError::OutputError)?;
        return Err(VersionError::Error(error));
    }
    parse_version_utf8(&command_output.stdout)
        .or_else(|utf8_err| parse_version_utf16(&command_output.stdout).map_err(|_| utf8_err))
}
