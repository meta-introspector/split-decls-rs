use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy)]
pub struct TokenTreesView<'a, S>(&'a [TokenTree<S>]);
