use serde::{Deserialize, Serialize};
use std::collections::HashMap;
unsafe impl<S: squeue::EntryMarker, C: cqueue::EntryMarker> Send for IoUring<S, C> {}
