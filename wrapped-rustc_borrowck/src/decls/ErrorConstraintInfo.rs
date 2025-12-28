macro_rules! ErrorConstraintInfo {
    () => {
        # [doc = " Information about the various region constraints involved in a borrow checker error."] # [derive (Clone , Debug)] pub (crate) struct ErrorConstraintInfo < 'tcx > { pub (super) fr : RegionVid , pub (super) outlived_fr : RegionVid , pub (super) category : ConstraintCategory < 'tcx > , pub (super) span : Span , }
    };
}

ErrorConstraintInfo!()