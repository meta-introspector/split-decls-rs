use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct RecursiveGuard(&'static LocalKey<AtomicBool>);
