macro_rules! constant_time_ne {
    () => {
        # [inline] # [must_use] fn constant_time_ne (a : & [u8] , b : & [u8]) -> u8 { assert ! (a . len () == b . len ()) ; let len = a . len () ; let a = & a [.. len] ; let b = & b [.. len] ; let mut tmp = 0 ; for i in 0 .. len { tmp |= a [i] ^ b [i] ; } optimizer_hide (tmp) }
    };
}

constant_time_ne!();