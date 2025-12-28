macro_rules! deps {
    () => {
        PResult!();
        Reject!();
        Cursor!();
    };
}

macro_rules! block_comment {
    () => {
        deps!();
        fn block_comment (input : Cursor) -> PResult < & str > { if ! input . starts_with ("/*") { return Err (Reject) ; } let mut depth = 0usize ; let bytes = input . as_bytes () ; let mut i = 0usize ; let upper = bytes . len () - 1 ; while i < upper { if bytes [i] == b'/' && bytes [i + 1] == b'*' { depth += 1 ; i += 1 ; } else if bytes [i] == b'*' && bytes [i + 1] == b'/' { depth -= 1 ; if depth == 0 { return Ok ((input . advance (i + 2) , & input . rest [.. i + 2])) ; } i += 1 ; } i += 1 ; } Err (Reject) }
    };
}

block_comment!();