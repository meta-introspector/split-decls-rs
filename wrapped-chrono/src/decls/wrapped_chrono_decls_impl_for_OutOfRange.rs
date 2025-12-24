use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl OutOfRange {
    const fn new() -> OutOfRange {
        OutOfRange { _private: () }
    }
}
