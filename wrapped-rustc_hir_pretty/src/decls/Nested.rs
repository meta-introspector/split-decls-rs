macro_rules! Nested {
    () => {
        pub enum Nested { Item (hir :: ItemId) , TraitItem (hir :: TraitItemId) , ImplItem (hir :: ImplItemId) , ForeignItem (hir :: ForeignItemId) , Body (hir :: BodyId) , BodyParamPat (hir :: BodyId , usize) , }
    };
}

Nested!()