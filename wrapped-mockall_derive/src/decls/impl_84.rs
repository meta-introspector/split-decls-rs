macro_rules! deps {
    () => {
        MockableModule!();
        Attrs!();
        MockableItem!();
        MockableStruct!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl From < (Attrs , Item) > for MockableItem { fn from ((attrs , item) : (Attrs , Item)) -> MockableItem { match item { Item :: Impl (item_impl) => MockableItem :: Struct (MockableStruct :: from (item_impl)) , Item :: Mod (item_mod) => MockableItem :: Module (MockableModule :: from (item_mod)) , Item :: Trait (trait_) => MockableItem :: Struct (MockableStruct :: from ((attrs , trait_))) , _ => panic ! ("automock does not support this item type") } } }
    };
}

impl_84!();