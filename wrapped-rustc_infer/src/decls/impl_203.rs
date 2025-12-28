macro_rules! deps {
    () => {
        Snapshot!();
        UndoLog!();
        InferCtxtUndoLogs!();
        RegionConstraintCollector!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'tcx > InferCtxtUndoLogs < 'tcx > { pub (crate) fn start_snapshot (& mut self) -> Snapshot < 'tcx > { self . num_open_snapshots += 1 ; Snapshot { undo_len : self . logs . len () , _marker : PhantomData } } pub (crate) fn region_constraints_in_snapshot (& self , s : & Snapshot < 'tcx > ,) -> impl Iterator < Item = & '_ region_constraints :: UndoLog < 'tcx > > + Clone { self . logs [s . undo_len ..] . iter () . filter_map (| log | match log { UndoLog :: RegionConstraintCollector (log) => Some (log) , _ => None , }) } pub (crate) fn opaque_types_in_snapshot (& self , s : & Snapshot < 'tcx >) -> bool { self . logs [s . undo_len ..] . iter () . any (| log | matches ! (log , UndoLog :: OpaqueTypes (..))) } fn assert_open_snapshot (& self , snapshot : & Snapshot < 'tcx >) { assert ! (self . logs . len () >= snapshot . undo_len) ; assert ! (self . num_open_snapshots > 0) ; } }
    };
}

impl_203!()