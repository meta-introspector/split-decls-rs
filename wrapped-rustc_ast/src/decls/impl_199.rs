macro_rules! deps {
    () => {
        PolyTraitRef!();
        TraitBoundModifiers!();
        Parens!();
        GenericParam!();
        Path!();
        TraitRef!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl PolyTraitRef { pub fn new (generic_params : ThinVec < GenericParam > , path : Path , modifiers : TraitBoundModifiers , span : Span , parens : Parens ,) -> Self { PolyTraitRef { bound_generic_params : generic_params , modifiers , trait_ref : TraitRef { path , ref_id : DUMMY_NODE_ID } , span , parens , } } }
    };
}

impl_199!();