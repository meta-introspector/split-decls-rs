macro_rules! into_i32 {
    () => {
        # [doc = " Convert usize into i32 without overflow."] # [doc = ""] # [doc = " This is needed to ensure when adjusting the exponent relative to"] # [doc = " the mantissa we do not overflow for comically-long exponents."] # [inline] fn into_i32 (value : usize) -> i32 { if value > i32 :: max_value () as usize { i32 :: max_value () } else { value as i32 } }
    };
}

into_i32!()