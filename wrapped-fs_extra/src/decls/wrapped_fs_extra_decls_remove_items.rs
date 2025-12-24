use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Removes a list of files or directories.
///
/// # Example
///
/// ```rust,ignore
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///
///  remove_items(&from_paths).unwrap();
/// ```
///
pub fn remove_items<P>(from_items: &[P]) -> Result<()>
where
    P: AsRef<Path>,
{
    for item in from_items {
        let item = item.as_ref();
        if item.is_dir() {
            dir::remove(item)?;
        } else {
            file::remove(item)?
        }
    }
    Ok(())
}
