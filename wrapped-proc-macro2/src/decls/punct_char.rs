macro_rules! deps {
    () => {
        PResult!();
        Cursor!();
        Reject!();
    };
}

macro_rules! punct_char {
    () => {
        deps!();
        fn punct_char (input : Cursor) -> PResult < char > { if input . starts_with ("//") || input . starts_with ("/*") { return Err (Reject) ; } let mut chars = input . chars () ; let Some (first) = chars . next () else { return Err (Reject) ; } ; let recognized = "~!@#$%^&*-=+|;:,<.>/?'" ; if recognized . contains (first) { Ok ((input . advance (first . len_utf8 ()) , first)) } else { Err (Reject) } }
    };
}

punct_char!();