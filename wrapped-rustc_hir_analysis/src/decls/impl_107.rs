macro_rules! deps {
    () => {
        GATArgsCollector!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'tcx > GATArgsCollector < 'tcx > { fn visit < T : TypeFoldable < TyCtxt < 'tcx > > > (gat : DefId , t : T ,) -> (FxIndexSet < (ty :: Region < 'tcx > , usize) > , FxIndexSet < (Ty < 'tcx > , usize) >) { let mut visitor = GATArgsCollector { gat , regions : FxIndexSet :: default () , types : FxIndexSet :: default () } ; t . visit_with (& mut visitor) ; (visitor . regions , visitor . types) } }
    };
}

impl_107!()