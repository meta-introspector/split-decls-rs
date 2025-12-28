macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        RegionRenumberer!();
    };
}

macro_rules! renumber_mir {
    () => {
        deps!();
        # [doc = " Replaces all free regions appearing in the MIR with fresh"] # [doc = " inference variables, returning the number of variables created."] # [instrument (skip (infcx , body , promoted) , level = "debug")] pub (crate) fn renumber_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & mut Body < 'tcx > , promoted : & mut IndexSlice < Promoted , Body < 'tcx > > ,) { debug ! (? body . arg_count) ; let mut renumberer = RegionRenumberer { infcx } ; for body in promoted . iter_mut () { renumberer . visit_body (body) ; } renumberer . visit_body (body) ; }
    };
}

renumber_mir!()