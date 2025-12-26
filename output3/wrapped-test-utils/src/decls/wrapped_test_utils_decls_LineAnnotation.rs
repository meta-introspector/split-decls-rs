use serde::{Deserialize, Serialize};
use std::collections::HashMap;
enum LineAnnotation {
    Annotation {
        range: TextRange,
        content: String,
        file: bool,
    },
    Continuation {
        offset: TextSize,
        content: String,
    },
}
