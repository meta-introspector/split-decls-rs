macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! factored_data_offset {
    () => {
        deps!();
        fn factored_data_offset (offset : i32 , factor : i8) -> Result < i32 > { let factor = i32 :: from (factor) ; let factored_offset = offset / factor ; if offset != factored_offset * factor { return Err (Error :: InvalidFrameDataOffset (offset)) ; } Ok (factored_offset) }
    };
}

factored_data_offset!();