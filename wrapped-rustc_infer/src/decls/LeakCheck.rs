macro_rules! deps {
    () => {
        MiniGraph!();
        RegionConstraintCollector!();
        SccUniverse!();
    };
}

macro_rules! LeakCheck {
    () => {
        deps!();
        struct LeakCheck < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , outer_universe : ty :: UniverseIndex , mini_graph : MiniGraph < 'tcx > , rcc : RegionConstraintCollector < 'a , 'tcx > , scc_placeholders : IndexVec < LeakCheckScc , Option < ty :: PlaceholderRegion > > , scc_universes : IndexVec < LeakCheckScc , SccUniverse < 'tcx > > , }
    };
}

LeakCheck!()