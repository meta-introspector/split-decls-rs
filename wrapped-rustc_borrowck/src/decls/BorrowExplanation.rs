macro_rules! deps {
    () => {
        OutlivesConstraint!();
        LaterUseKind!();
        RegionName!();
    };
}

macro_rules! BorrowExplanation {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum BorrowExplanation < 'tcx > { UsedLater (Local , LaterUseKind , Span , Option < Span >) , UsedLaterInLoop (LaterUseKind , Span , Option < Span >) , UsedLaterWhenDropped { drop_loc : Location , dropped_local : Local , should_note_order : bool , } , MustBeValidFor { category : ConstraintCategory < 'tcx > , from_closure : bool , span : Span , region_name : RegionName , opt_place_desc : Option < String > , path : Vec < OutlivesConstraint < 'tcx > > , } , Unexplained , }
    };
}

BorrowExplanation!()