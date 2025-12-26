use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn static_lt() -> Lifetime {
    Lifetime::new("'static", Span::call_site())
}
