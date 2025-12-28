macro_rules! constant_time_ne_n {
    () => {
        # [inline] # [must_use] fn constant_time_ne_n < const N : usize > (a : & [u8 ; N] , b : & [u8 ; N]) -> u8 { let mut tmp = 0 ; for i in 0 .. N { tmp |= a [i] ^ b [i] ; } optimizer_hide (tmp) }
    };
}

constant_time_ne_n!()