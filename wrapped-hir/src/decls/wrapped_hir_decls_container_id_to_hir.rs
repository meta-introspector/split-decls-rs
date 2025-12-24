use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn container_id_to_hir(c: ItemContainerId) -> ItemContainer {
    match c {
        ItemContainerId::ExternBlockId(id) => {
            ItemContainer::ExternBlock(ExternBlock { id })
        }
        ItemContainerId::ModuleId(id) => ItemContainer::Module(Module { id }),
        ItemContainerId::ImplId(id) => ItemContainer::Impl(Impl { id }),
        ItemContainerId::TraitId(id) => ItemContainer::Trait(Trait { id }),
    }
}
