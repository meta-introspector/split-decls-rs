use serde::{Deserialize, Serialize};
use std::collections::HashMap;
thread_local!(static IN_SCOPE : RefCell < bool > = const { RefCell::new(false) });
