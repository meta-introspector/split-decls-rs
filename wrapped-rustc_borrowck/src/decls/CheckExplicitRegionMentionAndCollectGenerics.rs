macro_rules! CheckExplicitRegionMentionAndCollectGenerics {
    () => {
        struct CheckExplicitRegionMentionAndCollectGenerics < 'tcx > { tcx : TyCtxt < 'tcx > , generics : & 'tcx ty :: Generics , offending_region_idx : usize , seen_opaques : FxIndexSet < DefId > , seen_lifetimes : FxIndexSet < DefId > , }
    };
}

CheckExplicitRegionMentionAndCollectGenerics!()