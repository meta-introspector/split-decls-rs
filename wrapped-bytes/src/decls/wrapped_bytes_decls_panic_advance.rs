use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Panic with a nice error message.
#[cold]
fn panic_advance(error_info: &TryGetError) -> ! {
    panic!(
        "advance out of bounds: the len is {} but advancing by {}",
        error_info.available, error_info.requested
    );
}
