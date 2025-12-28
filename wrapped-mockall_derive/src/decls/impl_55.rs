macro_rules! deps {
    () => {
        MockItemModule!();
        MockItem!();
        MockableItem!();
        MockItemStruct!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < MockableItem > for MockItem { fn from (mockable : MockableItem) -> MockItem { match mockable { MockableItem :: Struct (s) => MockItem :: Struct (MockItemStruct :: from (s)) , MockableItem :: Module (mod_) => MockItem :: Module (MockItemModule :: from (mod_)) } } }
    };
}

impl_55!()