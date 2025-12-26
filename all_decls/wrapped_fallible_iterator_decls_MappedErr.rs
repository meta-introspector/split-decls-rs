use serde::{Deserialize, Serialize};
use std::collections::HashMap;
enum MappedErr<T, U> {
    It(T),
    Fold(U),
}
