macro_rules! ExistingDerives {
    () => {
        pub (crate) type ExistingDerives = FxHashSet < hir :: Macro > ;
    };
}

ExistingDerives!()