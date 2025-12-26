use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn split_nonutf8_once(b: &OsStr) -> (&str, Option<&OsStr>) {
    match b.try_str() {
        Ok(s) => (s, None),
        Err(err) => {
            let (valid, after_valid) = unsafe { ext::split_at(b, err.valid_up_to()) };
            let valid = valid.try_str().unwrap();
            (valid, Some(after_valid))
        }
    }
}
