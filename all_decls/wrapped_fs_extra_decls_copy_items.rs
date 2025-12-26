use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Copies a list of directories and files to another place recursively. This function will
/// also copy the permission bits of the original files to destination files (not for
/// directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these case:
///
/// * List `from` contains  file or directory does not exist.
///
/// * List `from` contains  file or directory with invalid name.
///
/// * The current process does not have the permission to access to file from `lists from` or
/// `to`.
///
/// # Example
///
/// ```rust,ignore
///  extern crate fs_extra;
///  use fs_extra::dir::copy;
///
///  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
///
///  // copy dir1 and file1.txt to target/dir1 and target/file1.txt
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///  copy_items(&from_paths, "target", &options)?;
/// ```
///
pub fn copy_items<P, Q>(from: &[P], to: Q, options: &dir::CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut result: u64 = 0;
    if options.content_only {
        err!(
            "Options 'content_only' not acccess for copy_items function",
            ErrorKind::Other
        );
    }
    for item in from {
        let item = item.as_ref();
        if item.is_dir() {
            result += dir::copy(item, &to, options)?;
        } else if let Some(file_name) = item.file_name() {
            if let Some(file_name) = file_name.to_str() {
                let file_options = file::CopyOptions {
                    overwrite: options.overwrite,
                    skip_exist: options.skip_exist,
                    ..Default::default()
                };
                result += file::copy(item, to.as_ref().join(file_name), &file_options)?;
            }
        } else {
            err!("Invalid file name", ErrorKind::InvalidFileName);
        }
    }
    Ok(result)
}
