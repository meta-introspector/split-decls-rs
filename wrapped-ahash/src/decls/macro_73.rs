macro_rules! deps {
    () => {
        Convert!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (all (feature = "compile-time-rng" , any (test , fuzzing)))] { # [inline] fn get_fixed_seeds () -> &'static [[u64 ; 4] ; 2] { use const_random :: const_random ; const RAND : [[u64 ; 4] ; 2] = [[const_random ! (u64) , const_random ! (u64) , const_random ! (u64) , const_random ! (u64) ,] , [const_random ! (u64) , const_random ! (u64) , const_random ! (u64) , const_random ! (u64) ,]] ; & RAND } } else if # [cfg (all (feature = "runtime-rng" , not (fuzzing)))] { # [inline] fn get_fixed_seeds () -> &'static [[u64 ; 4] ; 2] { use crate :: convert :: Convert ; static SEEDS : OnceBox < [[u64 ; 4] ; 2] > = OnceBox :: new () ; SEEDS . get_or_init (|| { let mut result : [u8 ; 64] = [0 ; 64] ; getrandom :: fill (& mut result) . expect ("getrandom::fill() failed.") ; Box :: new (result . convert ()) }) } } else if # [cfg (feature = "compile-time-rng")] { # [inline] fn get_fixed_seeds () -> &'static [[u64 ; 4] ; 2] { use const_random :: const_random ; const RAND : [[u64 ; 4] ; 2] = [[const_random ! (u64) , const_random ! (u64) , const_random ! (u64) , const_random ! (u64) ,] , [const_random ! (u64) , const_random ! (u64) , const_random ! (u64) , const_random ! (u64) ,]] ; & RAND } } else { # [inline] fn get_fixed_seeds () -> &'static [[u64 ; 4] ; 2] { & [PI_U64X4 , PI2] } } }
    };
}

macro_73!()