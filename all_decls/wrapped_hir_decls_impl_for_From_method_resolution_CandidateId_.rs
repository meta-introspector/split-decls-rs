use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<method_resolution::CandidateId> for AssocItem {
    fn from(value: method_resolution::CandidateId) -> Self {
        match value {
            method_resolution::CandidateId::FunctionId(id) => AssocItem::Function(Function { id }),
            method_resolution::CandidateId::ConstId(id) => AssocItem::Const(Const { id }),
        }
    }
}
