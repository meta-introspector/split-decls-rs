macro_rules! deps {
    () => {
        RegionSnapshot!();
        Snapshot!();
    };
}

macro_rules! CombinedSnapshot {
    () => {
        deps!();
        # [must_use = "once you start a snapshot, you should always consume it"] pub struct CombinedSnapshot < 'tcx > { pub (super) undo_snapshot : Snapshot < 'tcx > , region_constraints_snapshot : RegionSnapshot , universe : ty :: UniverseIndex , }
    };
}

CombinedSnapshot!()