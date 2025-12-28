macro_rules! GATArgsCollector {
    () => {
        # [doc = " TypeVisitor that looks for uses of GATs like"] # [doc = " `<P0 as Trait<P1..Pn>>::GAT<Pn..Pm>` and adds the arguments `P0..Pm` into"] # [doc = " the two vectors, `regions` and `types` (depending on their kind). For each"] # [doc = " parameter `Pi` also track the index `i`."] struct GATArgsCollector < 'tcx > { gat : DefId , regions : FxIndexSet < (ty :: Region < 'tcx > , usize) > , types : FxIndexSet < (Ty < 'tcx > , usize) > , }
    };
}

GATArgsCollector!()