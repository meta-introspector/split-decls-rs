use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const _: () = {
    assert!(WS.len() == N_NEWLINES + N_SPACES);
    assert!(WS.as_bytes()[N_NEWLINES - 1] == b'\n');
    assert!(WS.as_bytes()[N_NEWLINES] == b' ');
};
