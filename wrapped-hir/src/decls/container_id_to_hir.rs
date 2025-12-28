macro_rules! deps {
    () => {
        Impl!();
        Trait!();
        ItemContainer!();
        ExternBlock!();
        Module!();
    };
}

macro_rules! container_id_to_hir {
    () => {
        deps!();
        fn container_id_to_hir (c : ItemContainerId) -> ItemContainer { match c { ItemContainerId :: ExternBlockId (id) => ItemContainer :: ExternBlock (ExternBlock { id }) , ItemContainerId :: ModuleId (id) => ItemContainer :: Module (Module { id }) , ItemContainerId :: ImplId (id) => ItemContainer :: Impl (Impl { id }) , ItemContainerId :: TraitId (id) => ItemContainer :: Trait (Trait { id }) , } }
    };
}

container_id_to_hir!();