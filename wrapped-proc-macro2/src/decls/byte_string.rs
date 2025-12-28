macro_rules! deps {
    () => {
        Cursor!();
        Reject!();
    };
}

macro_rules! byte_string {
    () => {
        deps!();
        fn byte_string (input : Cursor) -> Result < Cursor , Reject > { if let Ok (input) = input . parse ("b\"") { cooked_byte_string (input) } else if let Ok (input) = input . parse ("br") { raw_byte_string (input) } else { Err (Reject) } }
    };
}

byte_string!()