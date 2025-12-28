macro_rules! deps {
    () => {
        RegionConstraintCollector!();
        CombinedSnapshot!();
        MiniGraph!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'tcx > MiniGraph < 'tcx > { fn new (region_constraints : & RegionConstraintCollector < '_ , 'tcx > , only_consider_snapshot : Option < & CombinedSnapshot < 'tcx > > ,) -> Self { let mut nodes = FxIndexMap :: default () ; let mut edges = Vec :: new () ; Self :: iterate_region_constraints (region_constraints , only_consider_snapshot , | target , source | { let source_node = Self :: add_node (& mut nodes , source) ; let target_node = Self :: add_node (& mut nodes , target) ; edges . push ((source_node , target_node)) ; } ,) ; let graph = VecGraph :: < _ , false > :: new (nodes . len () , edges) ; let sccs = Sccs :: new (& graph) ; Self { nodes , sccs } } # [doc = " Invokes `each_edge(R1, R2)` for each edge where `R2: R1`"] fn iterate_region_constraints (region_constraints : & RegionConstraintCollector < '_ , 'tcx > , only_consider_snapshot : Option < & CombinedSnapshot < 'tcx > > , mut each_edge : impl FnMut (ty :: Region < 'tcx > , ty :: Region < 'tcx >) ,) { if let Some (snapshot) = only_consider_snapshot { for undo_entry in region_constraints . undo_log . region_constraints_in_snapshot (& snapshot . undo_snapshot) { match undo_entry { & AddConstraint (i) => { let c = region_constraints . data () . constraints [i] . 0 ; each_edge (c . sub , c . sup) ; } & AddVerify (i) => span_bug ! (region_constraints . data () . verifys [i] . origin . span () , "we never add verifications while doing higher-ranked things" ,) , & AddCombination (..) | & AddVar (..) => { } } } } else { region_constraints . data () . constraints . iter () . for_each (| (c , _) | each_edge (c . sub , c . sup)) } } fn add_node (nodes : & mut FxIndexMap < ty :: Region < 'tcx > , LeakCheckNode > , r : ty :: Region < 'tcx > ,) -> LeakCheckNode { let l = nodes . len () ; * nodes . entry (r) . or_insert (LeakCheckNode :: new (l)) } }
    };
}

impl_120!();