// Generated macro for rndq (function)
macro_rules! Depcrate_compress_longrndq {
() => {
// Module: crate::compress_long
// Provides: {"rndq"}
// Dependencies: {}
# [inline (always)] fn rndq (mut x : [u64 ; COLS] , r : u64) -> [u64 ; COLS] { for i in 0 .. COLS { x [i] ^= u64 :: MAX . wrapping_sub ((i as u64) << 4) ^ r ; } [column (& x , [1 , 3 , 5 , 11 , 0 , 2 , 4 , 6]) , column (& x , [2 , 4 , 6 , 12 , 1 , 3 , 5 , 7]) , column (& x , [3 , 5 , 7 , 13 , 2 , 4 , 6 , 8]) , column (& x , [4 , 6 , 8 , 14 , 3 , 5 , 7 , 9]) , column (& x , [5 , 7 , 9 , 15 , 4 , 6 , 8 , 10]) , column (& x , [6 , 8 , 10 , 0 , 5 , 7 , 9 , 11]) , column (& x , [7 , 9 , 11 , 1 , 6 , 8 , 10 , 12]) , column (& x , [8 , 10 , 12 , 2 , 7 , 9 , 11 , 13]) , column (& x , [9 , 11 , 13 , 3 , 8 , 10 , 12 , 14]) , column (& x , [10 , 12 , 14 , 4 , 9 , 11 , 13 , 15]) , column (& x , [11 , 13 , 15 , 5 , 10 , 12 , 14 , 0]) , column (& x , [12 , 14 , 0 , 6 , 11 , 13 , 15 , 1]) , column (& x , [13 , 15 , 1 , 7 , 12 , 14 , 0 , 2]) , column (& x , [14 , 0 , 2 , 8 , 13 , 15 , 1 , 3]) , column (& x , [15 , 1 , 3 , 9 , 14 , 0 , 2 , 4]) , column (& x , [0 , 2 , 4 , 10 , 15 , 1 , 3 , 5]) ,] }
};
}
