macro_rules! DropData {
    () => {
        struct DropData < 'tcx > { dropck_result : DropckOutlivesResult < 'tcx > , region_constraint_data : Option < & 'tcx QueryRegionConstraints < 'tcx > > , }
    };
}

DropData!()