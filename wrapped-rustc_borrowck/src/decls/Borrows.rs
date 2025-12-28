macro_rules! deps {
    () => {
        BorrowData!();
        BorrowSet!();
    };
}

macro_rules! Borrows {
    () => {
        deps!();
        # [doc = " `Borrows` stores the data used in the analyses that track the flow"] # [doc = " of borrows."] # [doc = ""] # [doc = " It uniquely identifies every borrow (`Rvalue::Ref`) by a"] # [doc = " `BorrowIndex`, and maps each such index to a `BorrowData`"] # [doc = " describing the borrow. These indexes are used for representing the"] # [doc = " borrows in compact bitvectors."] pub struct Borrows < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , borrow_set : & 'a BorrowSet < 'tcx > , borrows_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }
    };
}

Borrows!();