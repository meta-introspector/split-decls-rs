macro_rules! deps {
    () => {
        IsTraitAssocItem!();
        ImportInfo!();
        FxIndexMap!();
        ItemInNs!();
    };
}

macro_rules! ImportMapIndex {
    () => {
        deps!();
        type ImportMapIndex = FxIndexMap < ItemInNs , (SmallVec < ImportInfo , 1 > , IsTraitAssocItem) > ;
    };
}

ImportMapIndex!();