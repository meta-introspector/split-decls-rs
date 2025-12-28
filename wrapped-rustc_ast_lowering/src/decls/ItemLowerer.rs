macro_rules! deps {
    () => {
        AstOwner!();
    };
}

macro_rules! ItemLowerer {
    () => {
        deps!();
        pub (super) struct ItemLowerer < 'a , 'hir > { pub (super) tcx : TyCtxt < 'hir > , pub (super) resolver : & 'a mut ResolverAstLowering , pub (super) ast_index : & 'a IndexSlice < LocalDefId , AstOwner < 'a > > , pub (super) owners : & 'a mut IndexVec < LocalDefId , hir :: MaybeOwner < 'hir > > , }
    };
}

ItemLowerer!();