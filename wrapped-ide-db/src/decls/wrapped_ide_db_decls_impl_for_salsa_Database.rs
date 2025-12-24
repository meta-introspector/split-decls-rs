use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[salsa_macros::db]
impl salsa::Database for RootDatabase {}
