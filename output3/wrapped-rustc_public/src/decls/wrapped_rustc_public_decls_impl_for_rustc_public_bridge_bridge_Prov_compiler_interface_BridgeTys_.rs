use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl rustc_public_bridge::bridge::Prov<compiler_interface::BridgeTys> for crate::ty::Prov {
    fn new(aid: crate::mir::alloc::AllocId) -> Self {
        Self(aid)
    }
}
