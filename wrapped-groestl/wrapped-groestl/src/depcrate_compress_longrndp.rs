// Generated macro for rndp (function)
macro_rules! Depcrate_compress_longrndp {
() => {
// Module: crate::compress_long
// Provides: {"rndp"}
// Dependencies: {}
# [inline (always)] fn rndp (mut x : [u64 ; COLS] , r : u64) -> [u64 ; COLS] { for i in 0 .. COLS { x [i] ^= ((i as u64) << 60) ^ r ; } [column (& x , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 11]) , column (& x , [1 , 2 , 3 , 4 , 5 , 6 , 7 , 12]) , column (& x , [2 , 3 , 4 , 5 , 6 , 7 , 8 , 13]) , column (& x , [3 , 4 , 5 , 6 , 7 , 8 , 9 , 14]) , column (& x , [4 , 5 , 6 , 7 , 8 , 9 , 10 , 15]) , column (& x , [5 , 6 , 7 , 8 , 9 , 10 , 11 , 0]) , column (& x , [6 , 7 , 8 , 9 , 10 , 11 , 12 , 1]) , column (& x , [7 , 8 , 9 , 10 , 11 , 12 , 13 , 2]) , column (& x , [8 , 9 , 10 , 11 , 12 , 13 , 14 , 3]) , column (& x , [9 , 10 , 11 , 12 , 13 , 14 , 15 , 4]) , column (& x , [10 , 11 , 12 , 13 , 14 , 15 , 0 , 5]) , column (& x , [11 , 12 , 13 , 14 , 15 , 0 , 1 , 6]) , column (& x , [12 , 13 , 14 , 15 , 0 , 1 , 2 , 7]) , column (& x , [13 , 14 , 15 , 0 , 1 , 2 , 3 , 8]) , column (& x , [14 , 15 , 0 , 1 , 2 , 3 , 4 , 9]) , column (& x , [15 , 0 , 1 , 2 , 3 , 4 , 5 , 10]) ,] }
};
}
