use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Uniquely identifies a node in the HIR of the current crate. It is
/// composed of the `owner`, which is the `LocalDefId` of the directly enclosing
/// `hir::Item`, `hir::TraitItem`, or `hir::ImplItem` (i.e., the closest "item-like"),
/// and the `local_id` which is unique within the given owner.
///
/// This two-level structure makes for more stable values: One can move an item
/// around within the source code, or add or remove stuff before it, without
/// the `local_id` part of the `HirId` changing, which is a very useful property in
/// incremental compilation where we have to persist things through changes to
/// the code base.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Encodable, Decodable, HashStable_Generic)]
#[rustc_pass_by_value]
pub struct HirId {
    pub owner: OwnerId,
    pub local_id: ItemLocalId,
}
