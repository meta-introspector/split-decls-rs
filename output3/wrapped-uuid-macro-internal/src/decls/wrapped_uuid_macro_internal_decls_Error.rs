use serde::{Deserialize, Serialize};
use std::collections::HashMap;
enum Error {
    NonStringLiteral,
    UuidParse(LitStr, error::Error),
}
