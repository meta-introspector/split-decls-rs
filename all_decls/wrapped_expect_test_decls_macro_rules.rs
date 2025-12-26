use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Creates an instance of `ExpectFile` from relative or absolute path:
///
/// ```
/// # use expect_test::expect_file;
/// expect_file!["./test_data/bar.html"];
/// ```
#[macro_export]
macro_rules! expect_file {
    [$path:expr] => {
        $crate::ExpectFile { path : std::path::PathBuf::from($path), position : file!(),
        }
    };
}
