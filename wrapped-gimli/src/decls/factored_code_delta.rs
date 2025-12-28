macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! factored_code_delta {
    () => {
        deps!();
        fn factored_code_delta (prev_offset : u32 , offset : u32 , factor : u8) -> Result < u32 > { if offset < prev_offset { return Err (Error :: InvalidFrameCodeOffset (offset)) ; } let delta = offset - prev_offset ; let factor = u32 :: from (factor) ; let factored_delta = delta / factor ; if delta != factored_delta * factor { return Err (Error :: InvalidFrameCodeOffset (offset)) ; } Ok (factored_delta) }
    };
}

factored_code_delta!()