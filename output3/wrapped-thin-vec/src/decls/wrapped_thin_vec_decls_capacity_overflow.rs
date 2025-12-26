use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cold]
fn capacity_overflow() -> ! {
    panic!("capacity overflow")
}
