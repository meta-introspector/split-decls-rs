use serde::{Deserialize, Serialize};
use std::collections::HashMap;
static RT: Lazy<Mutex<Runtime>> = Lazy::new(Default::default);
