macro_rules! deps {
    () => {
        ItemInNs!();
        FxIndexMap!();
        ImportInfo!();
        IsTraitAssocItem!();
    };
}

macro_rules! ImportMapIndex {
    () => {
        deps!();
        type ImportMapIndex = FxIndexMap < ItemInNs , (SmallVec < ImportInfo , 1 > , IsTraitAssocItem) > ;
    };
}

ImportMapIndex!()