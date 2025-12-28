macro_rules! deps {
    () => {
        Side!();
        Hunk!();
    };
}

macro_rules! write_hunks {
    () => {
        deps!();
        pub fn write_hunks (hunks : & [Hunk] , input : & InternedInput < & [u8] > , current_tokens : & [Token] , out : & mut Vec < u8 > ,) { for hunk in hunks { let (tokens , range) = match hunk . side { Side :: Current => (current_tokens , & hunk . after) , Side :: Other => (input . after . as_slice () , & hunk . after) , Side :: Ancestor => (input . before . as_slice () , & hunk . before) , } ; write_tokens (& input . interner , & tokens [usize_range (range)] , out) ; } }
    };
}

write_hunks!()