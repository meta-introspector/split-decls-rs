use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn parse_version(version_str: &str) -> Result<Version, Option<ParseIntError>> {
    let mut words = version_str.split_whitespace().skip(1);
    let mut version = words.next().ok_or(None)?.split('.');
    let major = version.next().ok_or(None)?.parse()?;
    let minor = version.next().ok_or(None)?.parse()?;
    let patchlevel = words.nth(1).ok_or(None)?.to_owned();
    Ok(Version {
        major,
        minor,
        patch: patchlevel,
    })
}
