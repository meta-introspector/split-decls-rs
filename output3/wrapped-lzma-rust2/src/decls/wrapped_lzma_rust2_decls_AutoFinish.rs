use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait for writers that finishes the stream on drop.
trait AutoFinish {
    /// Finish writing the stream without error handling.
    fn finish_ignore_error(self);
}
