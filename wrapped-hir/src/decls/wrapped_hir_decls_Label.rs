use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Label {
    pub(crate) parent: DefWithBodyId,
    pub(crate) label_id: LabelId,
}
