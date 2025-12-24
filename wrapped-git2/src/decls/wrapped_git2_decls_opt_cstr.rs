use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn opt_cstr<T: IntoCString>(o: Option<T>) -> Result<Option<CString>, Error> {
    match o {
        Some(s) => s.into_c_string().map(Some),
        None => Ok(None),
    }
}
