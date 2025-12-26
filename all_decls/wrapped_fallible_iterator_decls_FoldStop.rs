use serde::{Deserialize, Serialize};
use std::collections::HashMap;
enum FoldStop<T, E> {
    Break(T),
    Err(E),
}
