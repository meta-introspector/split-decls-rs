macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! nanosecond_fixed {
    () => {
        deps!();
        # [doc = " Tries to consume a fixed number of digits as a fractional second."] # [doc = " Returns the number of whole nanoseconds (0--999,999,999)."] pub (super) fn nanosecond_fixed (s : & str , digits : usize) -> ParseResult < (& str , i64) > { let (s , v) = number (s , digits , digits) ? ; static SCALE : [i64 ; 10] = [0 , 100_000_000 , 10_000_000 , 1_000_000 , 100_000 , 10_000 , 1_000 , 100 , 10 , 1] ; let v = v . checked_mul (SCALE [digits]) . ok_or (OUT_OF_RANGE) ? ; Ok ((s , v)) }
    };
}

nanosecond_fixed!();