use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct WriteInput {
    dst: Expr,
    rest: TokenStream,
}
