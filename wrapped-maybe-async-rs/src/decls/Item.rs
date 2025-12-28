macro_rules! Item {
    () => {
        pub enum Item { Trait (ItemTrait) , Impl (ItemImpl) , Fn (ItemFn) , Static (ItemStatic) , }
    };
}

Item!()