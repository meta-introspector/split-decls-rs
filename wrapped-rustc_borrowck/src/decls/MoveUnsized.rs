macro_rules! MoveUnsized {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_move_unsized , code = E0161)] pub (crate) struct MoveUnsized < 'tcx > { pub ty : Ty < 'tcx > , # [primary_span] # [label] pub span : Span , }
    };
}

MoveUnsized!()