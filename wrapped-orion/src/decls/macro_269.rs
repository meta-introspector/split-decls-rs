macro_rules! macro_269 {
    () => {
        construct_tag ! { # [doc = " A type to represent the `Tag` that BLAKE2b returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `slice` is greater than 64 bytes."] (Tag , test_tag , 1 , BLAKE2B_OUTSIZE) }
    };
}

macro_269!()