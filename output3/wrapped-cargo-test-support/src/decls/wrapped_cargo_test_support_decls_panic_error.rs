use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `panic!`, reporting the specified error , see also [`t!`]
#[track_caller]
pub fn panic_error(what: &str, err: impl Into<anyhow::Error>) -> ! {
    let err = err.into();
    pe(what, err);
    #[track_caller]
    fn pe(what: &str, err: anyhow::Error) -> ! {
        let mut result = format!("{}\nerror: {}", what, err);
        for cause in err.chain().skip(1) {
            let _ = writeln!(result, "\nCaused by:");
            let _ = write!(result, "{}", cause);
        }
        panic!("\n{}", result);
    }
}
