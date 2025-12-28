macro_rules! deps {
    () => {
        ImplTrait!();
    };
}

macro_rules! ImplTraitIdx {
    () => {
        deps!();
        pub type ImplTraitIdx < 'db > = Idx < ImplTrait < 'db > > ;
    };
}

ImplTraitIdx!()