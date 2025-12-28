macro_rules! DefaultedParam {
    () => {
        type DefaultedParam = Either < hir :: TypeParam , hir :: ConstParam > ;
    };
}

DefaultedParam!();