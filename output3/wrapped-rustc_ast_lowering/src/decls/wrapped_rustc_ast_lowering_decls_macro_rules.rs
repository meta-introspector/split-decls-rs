use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! arena_vec {
    ($this:expr; $($x:expr),*) => {
        $this .arena.alloc_from_iter([$($x),*])
    };
}
