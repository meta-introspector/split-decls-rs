macro_rules! deps {
    () => {
        ItemInNs!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl From < ItemInNs > for hir_def :: item_scope :: ItemInNs { fn from (it : ItemInNs) -> Self { match it { ItemInNs :: Types (it) => Self :: Types (it . into ()) , ItemInNs :: Values (it) => Self :: Values (it . into ()) , ItemInNs :: Macros (it) => Self :: Macros (it . into ()) , } } }
    };
}

impl_42!()