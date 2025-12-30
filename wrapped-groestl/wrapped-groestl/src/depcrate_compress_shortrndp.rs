// Generated macro for rndp (function)
macro_rules! Depcrate_compress_shortrndp {
() => {
// Module: crate::compress_short
// Provides: {"rndp"}
// Dependencies: {}
# [inline (always)] fn rndp (mut x : [u64 ; COLS] , r : u64) -> [u64 ; COLS] { for i in 0 .. COLS { x [i] ^= ((i as u64) << 60) ^ r ; } [column (& x , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) , column (& x , [1 , 2 , 3 , 4 , 5 , 6 , 7 , 0]) , column (& x , [2 , 3 , 4 , 5 , 6 , 7 , 0 , 1]) , column (& x , [3 , 4 , 5 , 6 , 7 , 0 , 1 , 2]) , column (& x , [4 , 5 , 6 , 7 , 0 , 1 , 2 , 3]) , column (& x , [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4]) , column (& x , [6 , 7 , 0 , 1 , 2 , 3 , 4 , 5]) , column (& x , [7 , 0 , 1 , 2 , 3 , 4 , 5 , 6]) ,] }
};
}
