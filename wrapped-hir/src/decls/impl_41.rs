macro_rules! deps {
    () => {
        ItemInNs!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl From < hir_def :: item_scope :: ItemInNs > for ItemInNs { fn from (it : hir_def :: item_scope :: ItemInNs) -> Self { match it { hir_def :: item_scope :: ItemInNs :: Types (it) => ItemInNs :: Types (it . into ()) , hir_def :: item_scope :: ItemInNs :: Values (it) => ItemInNs :: Values (it . into ()) , hir_def :: item_scope :: ItemInNs :: Macros (it) => ItemInNs :: Macros (it . into ()) , } } }
    };
}

impl_41!()