macro_rules! deps {
    () => {
        BufferedDiag!();
    };
}

macro_rules! BorrowckDiagnosticsBuffer {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct BorrowckDiagnosticsBuffer < 'infcx , 'tcx > { # [doc = " This field keeps track of move errors that are to be reported for given move indices."] # [doc = ""] # [doc = " There are situations where many errors can be reported for a single move out (see"] # [doc = " #53807) and we want only the best of those errors."] # [doc = ""] # [doc = " The `report_use_of_moved_or_uninitialized` function checks this map and replaces the"] # [doc = " diagnostic (if there is one) if the `Place` of the error being reported is a prefix of"] # [doc = " the `Place` of the previous most diagnostic. This happens instead of buffering the"] # [doc = " error. Once all move errors have been reported, any diagnostics in this map are added"] # [doc = " to the buffer to be emitted."] # [doc = ""] # [doc = " `BTreeMap` is used to preserve the order of insertions when iterating. This is necessary"] # [doc = " when errors in the map are being re-added to the error buffer so that errors with the"] # [doc = " same primary span come out in a consistent order."] buffered_move_errors : BTreeMap < Vec < MoveOutIndex > , (PlaceRef < 'tcx > , Diag < 'infcx >) > , buffered_mut_errors : FxIndexMap < Span , (Diag < 'infcx > , usize) > , # [doc = " Buffer of diagnostics to be reported. A mixture of error and non-error diagnostics."] buffered_diags : Vec < BufferedDiag < 'infcx > > , }
    };
}

BorrowckDiagnosticsBuffer!()