macro_rules! Parsed {
    () => {
        # [doc = " A parsed finite and unsigned floating point number."] struct Parsed { # [doc = " Absolute value sig * 2^exp"] sig : u128 , exp : i32 , }
    };
}

Parsed!();