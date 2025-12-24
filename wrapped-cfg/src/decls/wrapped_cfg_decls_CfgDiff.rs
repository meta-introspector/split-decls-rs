use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CfgDiff {
    enable: Vec<CfgAtom>,
    disable: Vec<CfgAtom>,
}
