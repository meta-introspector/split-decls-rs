macro_rules! deps {
    () => {
        Result!();
        DebugTypeSignature!();
        Reader!();
    };
}

macro_rules! parse_type_signature {
    () => {
        deps!();
        # [doc = " Parse a type unit header's unique type signature. Callers should handle"] # [doc = " unique-ness checking."] fn parse_type_signature < R : Reader > (input : & mut R) -> Result < DebugTypeSignature > { input . read_u64 () . map (DebugTypeSignature) }
    };
}

parse_type_signature!();