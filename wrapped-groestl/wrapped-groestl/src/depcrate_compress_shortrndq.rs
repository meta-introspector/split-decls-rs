// Generated macro for rndq (function)
macro_rules! Depcrate_compress_shortrndq {
() => {
// Module: crate::compress_short
// Provides: {"rndq"}
// Dependencies: {}
# [inline (always)] fn rndq (mut x : [u64 ; COLS] , r : u64) -> [u64 ; COLS] { for i in 0 .. COLS { x [i] ^= u64 :: MAX . wrapping_sub ((i as u64) << 4) ^ r ; } [column (& x , [1 , 3 , 5 , 7 , 0 , 2 , 4 , 6]) , column (& x , [2 , 4 , 6 , 0 , 1 , 3 , 5 , 7]) , column (& x , [3 , 5 , 7 , 1 , 2 , 4 , 6 , 0]) , column (& x , [4 , 6 , 0 , 2 , 3 , 5 , 7 , 1]) , column (& x , [5 , 7 , 1 , 3 , 4 , 6 , 0 , 2]) , column (& x , [6 , 0 , 2 , 4 , 5 , 7 , 1 , 3]) , column (& x , [7 , 1 , 3 , 5 , 6 , 0 , 2 , 4]) , column (& x , [0 , 2 , 4 , 6 , 7 , 1 , 3 , 5]) ,] }
};
}
