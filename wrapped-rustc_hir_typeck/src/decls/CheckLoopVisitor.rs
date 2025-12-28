macro_rules! deps {
    () => {
        Context!();
        BlockInfo!();
    };
}

macro_rules! CheckLoopVisitor {
    () => {
        deps!();
        # [derive (Clone)] struct CheckLoopVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , cx_stack : Vec < Context > , block_breaks : BTreeMap < Span , BlockInfo > , }
    };
}

CheckLoopVisitor!()