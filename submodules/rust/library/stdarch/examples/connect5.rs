mkuse!{use rand :: seq :: SliceRandom ;}
mkuse!{use rand :: thread_rng ;}
mkuse!{use std :: cmp ;}
mkuse!{use std :: time :: Instant ;}
mkuse!{# [cfg (target_arch = "x86")] use core_arch :: arch :: x86 :: * ;}
mkuse!{# [cfg (target_arch = "x86_64")] use core_arch :: arch :: x86_64 :: * ;}
mkuse!{# [cfg (target_arch = "x86")] use std :: is_x86_feature_detected ;}
mkuse!{# [cfg (target_arch = "x86_64")] use std :: is_x86_feature_detected ;}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq)] pub enum Color { Black = 0 , White = 1 , Empty = 2 , Border = 3 , }}}
mkitem!{type Square = i32 ;}
mkitem!{type Move = i32 ;}
mkitem!{type Side = Color ;}
mkitem!{const FILE_SIZE : i32 = 15 ;}
mkitem!{const RANK_SIZE : i32 = 15 ;}
mkitem!{const SQUARE_SIZE : i32 = (FILE_SIZE + 1) * (FILE_SIZE + 4) + 16 + 4 ;}
mkitem!{const EVAL_INF : i32 = FILE_SIZE * RANK_SIZE * 100 ;}
mkitem!{const MOVE_NONE : Move = - 1 ;}
mkitem!{const SCORE_NONE : i32 = - EVAL_INF - 1 ;}
mkitem!{# [doc = " DIRECTION 0: left to right\\"] # [doc = " DIRECTION 1: top to bottom\\"] # [doc = " DIRECTION 2: top left to bottom right\\"] # [doc = " DIRECTION 3: top right to bottom left"] # [rustfmt :: skip] # [allow (clippy :: identity_op)] const DIRECTION : [[i32 ; 5] ; 4] = [[1 , 2 , 3 , 4 , 5] , [1 * (FILE_SIZE + 1) , 2 * (FILE_SIZE + 1) , 3 * (FILE_SIZE + 1) , 4 * (FILE_SIZE + 1) , 5 * (FILE_SIZE + 1)] , [1 * (FILE_SIZE + 2) , 2 * (FILE_SIZE + 2) , 3 * (FILE_SIZE + 2) , 4 * (FILE_SIZE + 2) , 5 * (FILE_SIZE + 2)] , [1 * (FILE_SIZE + 0) , 2 * (FILE_SIZE + 0) , 3 * (FILE_SIZE + 0) , 4 * (FILE_SIZE + 0) , 5 * (FILE_SIZE + 0)]] ;}
mkitem!{# [doc = " A table to encode each location to a value in bit 31-0 in the bitboard for 4 direction"] # [rustfmt :: skip] const MAPMOVEVALUE : [[i32 ; 239] ; 4] = [[1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17 , 0 , 1 << 31 , 1 << 30 , 1 << 29 , 1 << 28 , 1 << 27 , 1 << 26 , 1 << 25 , 1 << 24 , 1 << 23 , 1 << 22 , 1 << 21 , 1 << 20 , 1 << 19 , 1 << 18 , 1 << 17] , [1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 1 << 31 , 0 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 1 << 30 , 0 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 1 << 29 , 0 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 1 << 28 , 0 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 1 << 27 , 0 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 1 << 26 , 0 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 1 << 25 , 0 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 1 << 24 , 0 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 1 << 23 , 0 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 1 << 22 , 0 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 1 << 21 , 0 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 1 << 20 , 0 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 1 << 19 , 0 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 1 << 18 , 0 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17 , 1 << 17] , [1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 0 , 0 , 0 , 0 , 0 , 1 << 15 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 0 , 0 , 0 , 0 , 1 << 15 , 1 << 14 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 0 , 0 , 0 , 1 << 15 , 1 << 14 , 1 << 13 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 0 , 0 , 1 << 15 , 1 << 14 , 1 << 13 , 1 << 12 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 0 , 1 << 15 , 1 << 14 , 1 << 13 , 1 << 12 , 1 << 11 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 0 , 1 << 9 , 1 << 14 , 1 << 13 , 1 << 12 , 1 << 11 , 1 << 10 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 0 , 1 << 8 , 1 << 8 , 1 << 13 , 1 << 12 , 1 << 11 , 1 << 10 , 1 << 9 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 0 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 12 , 1 << 11 , 1 << 10 , 1 << 9 , 1 << 8 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 0 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 11 , 1 << 10 , 1 << 9 , 1 << 8 , 1 << 7 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 0 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 10 , 1 << 9 , 1 << 8 , 1 << 7 , 1 << 6 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 0 , 0 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 9 , 1 << 8 , 1 << 7 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 0 , 0 , 0 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 8 , 1 << 7 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 3 , 1 << 3 , 1 << 3 , 0 , 0 , 0 , 0 , 1 << 2 , 1 << 2 , 1 << 2 , 1 << 2 , 1 << 2 , 1 << 7 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 3 , 1 << 2 , 1 << 2 , 0 , 0 , 0 , 0 , 0 , 1 << 1 , 1 << 1 , 1 << 1 , 1 << 1 , 1 << 1 , 1 << 6 , 1 << 5 , 1 << 4 , 1 << 3 , 1 << 2 , 1 << 1] , [0 , 0 , 0 , 0 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 1 << 15 , 0 , 0 , 0 , 0 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 14 , 1 << 15 , 0 , 0 , 0 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 13 , 1 << 14 , 1 << 15 , 0 , 0 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 12 , 1 << 13 , 1 << 14 , 1 << 15 , 0 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 11 , 1 << 12 , 1 << 13 , 1 << 14 , 1 << 15 , 0 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 10 , 1 << 11 , 1 << 12 , 1 << 13 , 1 << 14 , 1 << 15 , 0 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 9 , 1 << 10 , 1 << 11 , 1 << 12 , 1 << 13 , 1 << 14 , 1 << 9 , 0 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 8 , 1 << 9 , 1 << 10 , 1 << 11 , 1 << 12 , 1 << 13 , 1 << 8 , 1 << 8 , 0 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 7 , 1 << 8 , 1 << 9 , 1 << 10 , 1 << 11 , 1 << 12 , 1 << 7 , 1 << 7 , 1 << 7 , 0 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 7 , 1 << 8 , 1 << 9 , 1 << 10 , 1 << 11 , 1 << 6 , 1 << 6 , 1 << 6 , 1 << 6 , 0 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 6 , 1 << 7 , 1 << 8 , 1 << 9 , 1 << 10 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 1 << 5 , 0 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 5 , 1 << 6 , 1 << 7 , 1 << 8 , 1 << 9 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 1 << 4 , 0 , 0 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 4 , 1 << 5 , 1 << 6 , 1 << 7 , 1 << 8 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 3 , 1 << 3 , 0 , 0 , 0 , 1 << 2 , 1 << 2 , 1 << 3 , 1 << 4 , 1 << 5 , 1 << 6 , 1 << 7 , 1 << 2 , 1 << 2 , 1 << 2 , 1 << 2 , 1 << 2 , 0 , 0 , 0 , 0 , 1 << 1 , 1 << 2 , 1 << 3 , 1 << 4 , 1 << 5 , 1 << 6 , 1 << 1 , 1 << 1 , 1 << 1 , 1 << 1 , 1 << 1 , 0 , 0 , 0 , 0]] ;}
mkitem!{# [doc = " A table to encode each location to an index in the bitboard for 4 direction"] # [rustfmt :: skip] const MAPMOVEIDX : [[i32 ; 239] ; 4] = [[0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 0 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 0 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 0 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 4 , 0 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 5 , 0 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 6 , 0 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 7 , 0 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 8 , 0 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 9 , 0 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 10 , 0 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 11 , 0 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 12 , 0 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 13 , 0 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14] , [10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 0 , 0 , 0 , 0 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 0 , 0 , 0 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 0 , 0 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 0 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 1 , 0 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 2 , 0 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 3 , 0 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 4 , 0 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 5 , 0 , 5 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 6 , 0 , 0 , 5 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 7 , 0 , 0 , 0 , 5 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 8 , 0 , 0 , 0 , 0 , 5 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10 , 9 , 0 , 0 , 0 , 0 , 0 , 5 , 4 , 3 , 2 , 1 , 15 , 14 , 13 , 12 , 11 , 10] , [0 , 0 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 0 , 0 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 0 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 0 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 0 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 0 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 0 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 0 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 0 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 0 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 5 , 0 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 5 , 0 , 0 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 5 , 0 , 0 , 0 , 9 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 5 , 0 , 0 , 0 , 0 , 10 , 11 , 12 , 13 , 14 , 15 , 1 , 2 , 3 , 4 , 5 , 0 , 0 , 0 , 0]] ;}
mkitem!{mkstruct!{# [doc = " Use one-dimensional array to store the board state. The location 0 is top left.\\"] # [doc = " 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  <b>15</b>\\"] # [doc = " 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  <b>31</b>\\"] # [doc = " ... \\"] # [doc = " position 15, 31, ... are Borders.\\"] # [doc = " position 0 is file 0, rank 0.\\"] # [doc = " position 17 is file 1, rank 1.\\"] # [doc = ""] # [doc = " Use a three-dimensional array to store the bitboard.\\"] # [doc = " The first dimension is color: Black, White and Empty.\\"] # [doc = " The second and third one are 2 x 512-bit. Direction 0 and 2 use the first 512-bit. Direction 1 and"] # [doc = " 3 use the second 512-bit.\\"] # [doc = " Each 512-bit is a 32-bit x 16 array. Direction 0 and 1 store at bit 31-16 and Direction 2 and 3 store at bit 15-0."] pub struct Pos { state : [Color ; SQUARE_SIZE as usize] , p_turn : Side , bitboard : [[[i32 ; 16] ; 2] ; 3] , }}}
mkitem!{mkimpl!{impl Pos { pub fn init (& mut self) { for i in 0 .. SQUARE_SIZE as usize { self . state [i] = Color :: Border ; } for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; self . state [sq as usize] = Color :: Empty ; } } self . p_turn = Color :: Black ; let black = Color :: Black as usize ; let white = Color :: White as usize ; let empty = Color :: Empty as usize ; for i in 0 .. 2 { for j in 0 .. 16 { self . bitboard [black] [i] [j] = 0 ; self . bitboard [white] [i] [j] = 0 ; self . bitboard [empty] [i] [j] = 0 ; } } for i in 0 .. 2 { # [rustfmt :: skip] for j in 0 .. FILE_SIZE as usize { self . bitboard [empty] [i] [j] = (1 << 31) | (1 << 30) | (1 << 29) | (1 << 28) | (1 << 27) | (1 << 26) | (1 << 25) | (1 << 24) | (1 << 23) | (1 << 22) | (1 << 21) | (1 << 20) | (1 << 19) | (1 << 18) | (1 << 17) ; } } # [rustfmt :: skip] for i in 0 .. 2 { self . bitboard [empty] [i] [0] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) ; self . bitboard [empty] [i] [1] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [2] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [3] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [4] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [5] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [6] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) ; self . bitboard [empty] [i] [7] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) ; self . bitboard [empty] [i] [8] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) ; self . bitboard [empty] [i] [9] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) ; self . bitboard [empty] [i] [10] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1) ; self . bitboard [empty] [i] [11] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) ; self . bitboard [empty] [i] [12] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) ; self . bitboard [empty] [i] [13] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) ; self . bitboard [empty] [i] [14] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5) ; self . bitboard [empty] [i] [15] |= (1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) ; } } pub fn do_move (& mut self , mv : Move) { let atk : Side = self . p_turn ; let def : Side = side_opp (atk) ; let mv = mv as usize ; let black = Color :: Black as usize ; let white = Color :: White as usize ; let empty = Color :: Empty as usize ; match self . p_turn { Color :: Black => { self . state [mv] = Color :: Black ; self . bitboard [black] [0] [MAPMOVEIDX [0] [mv] as usize] |= MAPMOVEVALUE [0] [mv] ; self . bitboard [empty] [0] [MAPMOVEIDX [0] [mv] as usize] ^= MAPMOVEVALUE [0] [mv] ; self . bitboard [black] [1] [MAPMOVEIDX [1] [mv] as usize] |= MAPMOVEVALUE [1] [mv] ; self . bitboard [empty] [1] [MAPMOVEIDX [1] [mv] as usize] ^= MAPMOVEVALUE [1] [mv] ; self . bitboard [black] [0] [MAPMOVEIDX [2] [mv] as usize] |= MAPMOVEVALUE [2] [mv] ; self . bitboard [empty] [0] [MAPMOVEIDX [2] [mv] as usize] ^= MAPMOVEVALUE [2] [mv] ; self . bitboard [black] [1] [MAPMOVEIDX [3] [mv] as usize] |= MAPMOVEVALUE [3] [mv] ; self . bitboard [empty] [1] [MAPMOVEIDX [3] [mv] as usize] ^= MAPMOVEVALUE [3] [mv] ; } Color :: White => { self . state [mv] = Color :: White ; self . bitboard [white] [0] [MAPMOVEIDX [0] [mv] as usize] |= MAPMOVEVALUE [0] [mv] ; self . bitboard [empty] [0] [MAPMOVEIDX [0] [mv] as usize] ^= MAPMOVEVALUE [0] [mv] ; self . bitboard [white] [1] [MAPMOVEIDX [1] [mv] as usize] |= MAPMOVEVALUE [1] [mv] ; self . bitboard [empty] [1] [MAPMOVEIDX [1] [mv] as usize] ^= MAPMOVEVALUE [1] [mv] ; self . bitboard [white] [0] [MAPMOVEIDX [2] [mv] as usize] |= MAPMOVEVALUE [2] [mv] ; self . bitboard [empty] [0] [MAPMOVEIDX [2] [mv] as usize] ^= MAPMOVEVALUE [2] [mv] ; self . bitboard [white] [1] [MAPMOVEIDX [3] [mv] as usize] |= MAPMOVEVALUE [3] [mv] ; self . bitboard [empty] [1] [MAPMOVEIDX [3] [mv] as usize] ^= MAPMOVEVALUE [3] [mv] ; } _ => panic ! { } , } self . p_turn = def ; } fn turn (& self) -> Side { self . p_turn } pub fn can_play (& self , from : Square) -> bool { self . state [from as usize] == Color :: Empty } }}}
mkitem!{mkstruct!{pub struct List { p_move : [Move ; (FILE_SIZE * RANK_SIZE) as usize] , p_size : i32 , }}}
mkitem!{mkimpl!{# [doc = " Use List to store legal moves."] impl List { pub fn clear (& mut self) { self . p_size = 0 ; } pub fn add (& mut self , mv : Move) { self . p_move [self . p_size as usize] = mv ; self . p_size += 1 ; } pub fn size (& self) -> i32 { self . p_size } pub fn shuffle (& mut self) { let mut rng = thread_rng () ; let num = self . p_size as usize ; self . p_move [.. num] . shuffle (& mut rng) ; } }}}

macro_rules! square_make_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function square_make in module {}", module_path!());
    };
}

mkfn!{
    square_make_introspect!();
    fn square_make (fl : i32 , rk : i32) -> Square { rk * (FILE_SIZE + 1) + fl }
}

macro_rules! side_opp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function side_opp in module {}", module_path!());
    };
}

mkfn!{
    side_opp_introspect!();
    fn side_opp (sd : Side) -> Side { match sd { Side :: White => Side :: Black , Side :: Black => Side :: White , _ => panic ! ("") , } }
}

macro_rules! pos_is_winner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_is_winner in module {}", module_path!());
    };
}

mkfn!{
    pos_is_winner_introspect!();
    fn pos_is_winner (pos : & Pos) -> bool { let current_side = side_opp (pos . p_turn) ; check_pattern5 (pos , current_side) }
}

macro_rules! pos_is_draw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_is_draw in module {}", module_path!());
    };
}

mkfn!{
    pos_is_draw_introspect!();
    fn pos_is_draw (pos : & Pos) -> bool { let mut found : bool = true ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; if pos . can_play (sq) { found = false ; break ; } if ! found { break ; } } } found && ! pos_is_winner (pos) }
}

macro_rules! pos_is_draw_avx512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_is_draw_avx512 in module {}", module_path!());
    };
}

mkfn!{
    pos_is_draw_avx512_introspect!();
    # [target_feature (enable = "avx512f,avx512bw,popcnt")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn pos_is_draw_avx512 (pos : & Pos) -> bool { let empty = Color :: Empty as usize ; let board0org = unsafe { _mm512_loadu_epi32 (& pos . bitboard [empty] [0] [0]) } ; let answer = _mm512_set1_epi32 (0) ; let temp_mask = _mm512_mask_cmpneq_epi32_mask (0b11111111_11111111 , answer , board0org) ; _popcnt32 (temp_mask as i32) == 0 && ! pos_is_winner_avx512 (pos) }
}

macro_rules! pos_is_end_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_is_end in module {}", module_path!());
    };
}

mkfn!{
    pos_is_end_introspect!();
    fn pos_is_end (pos : & Pos) -> bool { pos_is_winner (pos) || pos_is_draw (pos) }
}

macro_rules! pos_disp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_disp in module {}", module_path!());
    };
}

mkfn!{
    pos_disp_introspect!();
    fn pos_disp (pos : & Pos) { for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; match pos . state [sq as usize] { Color :: Black => print ! ("# ") , Color :: White => print ! ("O ") , Color :: Empty => print ! ("- ") , Color :: Border => print ! ("| ") , } } println ! () ; } match pos . turn () { Color :: Black => println ! ("black to play") , Color :: White => println ! ("white to play") , _ => panic ! () , } }
}

macro_rules! gen_moves_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_moves in module {}", module_path!());
    };
}

mkfn!{
    gen_moves_introspect!();
    fn gen_moves (list : & mut List , pos : & Pos) { list . clear () ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; if pos . can_play (sq) { list . add (sq) ; } } } }
}

macro_rules! search_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search in module {}", module_path!());
    };
}

mkfn!{
    search_introspect!();
    # [doc = " AI: use Minimax search with alpha-beta pruning"] # [allow (clippy :: manual_range_contains)] fn search (pos : & Pos , alpha : i32 , beta : i32 , depth : i32 , _ply : i32) -> i32 { assert ! (- EVAL_INF <= alpha && alpha < beta && beta <= EVAL_INF) ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { unsafe { if pos_is_winner_avx512 (pos) { return - EVAL_INF + _ply ; } if pos_is_draw_avx512 (pos) { return 0 ; } } } else { if pos_is_winner (pos) { return - EVAL_INF + _ply ; } if pos_is_draw (pos) { return 0 ; } } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { if pos_is_winner (pos) { return - EVAL_INF + _ply ; } if pos_is_draw (pos) { return 0 ; } } if depth == 0 { return eval (pos , _ply) ; } let p_move_new : [Move ; (FILE_SIZE * RANK_SIZE) as usize] = [0 ; (FILE_SIZE * RANK_SIZE) as usize] ; let mut list = List { p_move : p_move_new , p_size : 0 , } ; let mut bm : Move = MOVE_NONE ; let mut bs : i32 = SCORE_NONE ; gen_moves (& mut list , pos) ; if _ply == 0 { list . shuffle () ; } for i in 0 .. list . size () { if bs < beta { let mv : Move = list . p_move [i as usize] ; let mut new_pos = Pos { state : pos . state , p_turn : pos . p_turn , bitboard : pos . bitboard , } ; new_pos . do_move (mv) ; let sc : i32 = - search (& new_pos , - beta , - cmp :: max (alpha , bs) , depth - 1 , _ply + 1) ; if sc > bs { bm = mv ; bs = sc ; } } } assert_ne ! (bm , MOVE_NONE) ; assert ! (bs >= - EVAL_INF && bs <= EVAL_INF) ; if _ply == 0 { bm } else { bs } }
}

macro_rules! eval_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval in module {}", module_path!());
    };
}

mkfn!{
    eval_introspect!();
    # [doc = " Evaluation function: give different scores to different patterns after a fixed depth."] fn eval (pos : & Pos , _ply : i32) -> i32 { let atk : Side = pos . turn () ; let def : Side = side_opp (atk) ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { if unsafe { check_patternlive4_avx512 (pos , def) } { return - 4096 ; } } else if check_patternlive4 (pos , def) { return - 4096 ; } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { if check_patternlive4 (pos , def) { return - 4096 ; } } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { if unsafe { check_patternlive4_avx512 (pos , atk) } { return 2560 ; } } else if check_patternlive4 (pos , atk) { return 2560 ; } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { if check_patternlive4 (pos , atk) { return 2560 ; } } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { if unsafe { check_patterndead4_avx512 (pos , atk) > 0 } { return 2560 ; } } else if check_patterndead4 (pos , atk) > 0 { return 2560 ; } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { if check_patterndead4 (pos , atk) > 0 { return 2560 ; } } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { unsafe { let n_c4 : i32 = check_patterndead4_avx512 (pos , def) ; let n_c3 : i32 = check_patternlive3_avx512 (pos , def) ; if n_c4 > 1 { return - 2048 ; } if n_c4 == 1 && n_c3 > 0 { return - 2048 ; } if check_patternlive3_avx512 (pos , atk) > 1 { return 2560 ; } if n_c3 > 1 { return - 2048 ; } } } else { let n_c4 : i32 = check_patterndead4 (pos , def) ; let n_c3 : i32 = check_patternlive3 (pos , def) ; if n_c4 > 1 { return - 2048 ; } if n_c4 == 1 && n_c3 > 0 { return - 2048 ; } if check_patternlive3 (pos , atk) > 1 { return 2560 ; } if n_c3 > 1 { return - 2048 ; } } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { let n_c4 : i32 = check_patterndead4 (pos , def) ; let n_c3 : i32 = check_patternlive3 (pos , def) ; if n_c4 > 1 { return - 2048 ; } if n_c4 == 1 && n_c3 > 0 { return - 2048 ; } if check_patternlive3 (pos , atk) > 1 { return 2560 ; } if n_c3 > 1 { return - 2048 ; } } 0 }
}

macro_rules! check_pattern5_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_pattern5 in module {}", module_path!());
    };
}

mkfn!{
    check_pattern5_introspect!();
    # [doc = " Check <b>OOOOO</b>"] fn check_pattern5 (pos : & Pos , sd : Side) -> bool { let mut n : i32 = 0 ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; for direction in & DIRECTION { let idx0 = sq ; let idx1 = sq + direction [0] ; let idx2 = sq + direction [1] ; let idx3 = sq + direction [2] ; let idx4 = sq + direction [3] ; let val0 = pos . state [idx0 as usize] ; let val1 = pos . state [idx1 as usize] ; let val2 = pos . state [idx2 as usize] ; let val3 = pos . state [idx3 as usize] ; let val4 = pos . state [idx4 as usize] ; # [rustfmt :: skip] if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1 ; } } } } n > 0 }
}

macro_rules! check_patternlive4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patternlive4 in module {}", module_path!());
    };
}

mkfn!{
    check_patternlive4_introspect!();
    # [doc = " Check <b>-OOOO-</b>"] fn check_patternlive4 (pos : & Pos , sd : Side) -> bool { let mut n : i32 = 0 ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; for direction in & DIRECTION { let idx0 = sq ; let idx1 = sq + direction [0] ; let idx2 = sq + direction [1] ; let idx3 = sq + direction [2] ; let idx4 = sq + direction [3] ; let idx5 = sq + direction [4] ; let val0 = pos . state [idx0 as usize] ; let val1 = pos . state [idx1 as usize] ; let val2 = pos . state [idx2 as usize] ; let val3 = pos . state [idx3 as usize] ; let val4 = pos . state [idx4 as usize] ; let val5 = pos . state [idx5 as usize] ; # [rustfmt :: skip] if val0 == Color :: Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd && val5 == Color :: Empty { n += 1 ; } } } } n > 0 }
}

macro_rules! check_patterndead4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patterndead4 in module {}", module_path!());
    };
}

mkfn!{
    check_patterndead4_introspect!();
    # [doc = " Check <b>OOOO-, OOO-O, OO-OO, O-OOO, -OOOO</b>"] fn check_patterndead4 (pos : & Pos , sd : Side) -> i32 { let mut n : i32 = 0 ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; for direction in & DIRECTION { let idx0 = sq ; let idx1 = sq + direction [0] ; let idx2 = sq + direction [1] ; let idx3 = sq + direction [2] ; let idx4 = sq + direction [3] ; let val0 = pos . state [idx0 as usize] ; let val1 = pos . state [idx1 as usize] ; let val2 = pos . state [idx2 as usize] ; let val3 = pos . state [idx3 as usize] ; let val4 = pos . state [idx4 as usize] ; # [rustfmt :: skip] if val0 == sd && val1 == sd && val2 == sd && val3 == sd && val4 == Color :: Empty { n += 1 ; } # [rustfmt :: skip] if val0 == sd && val1 == sd && val2 == sd && val3 == Color :: Empty && val4 == sd { n += 1 ; } # [rustfmt :: skip] if val0 == sd && val1 == sd && val2 == Color :: Empty && val3 == sd && val4 == sd { n += 1 ; } # [rustfmt :: skip] if val0 == sd && val1 == Color :: Empty && val2 == sd && val3 == sd && val4 == sd { n += 1 ; } # [rustfmt :: skip] if val0 == Color :: Empty && val1 == sd && val2 == sd && val3 == sd && val4 == sd { n += 1 ; } } } } n }
}

macro_rules! check_patternlive3_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patternlive3 in module {}", module_path!());
    };
}

mkfn!{
    check_patternlive3_introspect!();
    # [doc = " Check <b>-OOO-, -OO-O-, -O-OO-</b>"] fn check_patternlive3 (pos : & Pos , sd : Side) -> i32 { let mut n : i32 = 0 ; for rk in 0 .. RANK_SIZE { for fl in 0 .. FILE_SIZE { let sq : Square = square_make (fl , rk) ; for direction in & DIRECTION { let idx0 = sq ; let idx1 = sq + direction [0] ; let idx2 = sq + direction [1] ; let idx3 = sq + direction [2] ; let idx4 = sq + direction [3] ; let idx5 = sq + direction [4] ; let val0 = pos . state [idx0 as usize] ; let val1 = pos . state [idx1 as usize] ; let val2 = pos . state [idx2 as usize] ; let val3 = pos . state [idx3 as usize] ; let val4 = pos . state [idx4 as usize] ; let val5 = pos . state [idx5 as usize] ; # [rustfmt :: skip] if val0 == Color :: Empty && val1 == sd && val2 == sd && val3 == sd && val4 == Color :: Empty { n += 1 ; } # [rustfmt :: skip] if val0 == Color :: Empty && val1 == sd && val2 == sd && val3 == Color :: Empty && val4 == sd && val5 == Color :: Empty { n += 1 ; } # [rustfmt :: skip] if val0 == Color :: Empty && val1 == sd && val2 == Color :: Empty && val3 == sd && val4 == sd && val5 == Color :: Empty { n += 1 ; } } } } n }
}

macro_rules! pos_is_winner_avx512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pos_is_winner_avx512 in module {}", module_path!());
    };
}

mkfn!{
    pos_is_winner_avx512_introspect!();
    # [target_feature (enable = "avx512f,avx512bw,popcnt")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn pos_is_winner_avx512 (pos : & Pos) -> bool { let current_side = side_opp (pos . p_turn) ; let coloridx = current_side as usize ; let board0org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [coloridx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [coloridx] [1] [0]) ,] } ; # [rustfmt :: skip] let answer = _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11)) ; # [rustfmt :: skip] let answer_mask : [__mmask32 ; 11] = [0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_10_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_10_10_10_10_10 , 0b00_11_11_11_11_11_11_11_11_11_10_10_10_10_11_10 , 0b00_10_11_11_11_11_11_11_11_10_10_10_10_11_11_10 , 0b00_10_10_11_11_11_11_11_10_10_10_10_11_11_11_10 , 0b00_10_10_10_11_11_11_10_10_10_10_11_11_11_11_10 , 0b00_10_10_10_10_11_10_10_10_10_11_11_11_11_11_10] ; let mut count_match : i32 = 0 ; for mut board0 in board0org { let boardf = _mm512_and_si512 (answer , board0) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [0] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; for i in 1 .. 11 { board0 = _mm512_slli_epi32 (board0 , 1) ; let boardf = _mm512_and_si512 (answer , board0) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [i] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; } } count_match > 0 }
}

macro_rules! check_patternlive4_avx512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patternlive4_avx512 in module {}", module_path!());
    };
}

mkfn!{
    check_patternlive4_avx512_introspect!();
    # [target_feature (enable = "avx512f,avx512bw,popcnt")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn check_patternlive4_avx512 (pos : & Pos , sd : Side) -> bool { let coloridx = sd as usize ; let emptyidx = Color :: Empty as usize ; # [rustfmt :: skip] let answer_color = _mm512_set1_epi16 ((1 << 14) | (1 << 13) | (1 << 12) | (1 << 11)) ; # [rustfmt :: skip] let answer_empty = _mm512_set1_epi16 ((1 << 15) | (1 << 10)) ; # [rustfmt :: skip] let answer = _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10)) ; # [rustfmt :: skip] let answer_mask : [__mmask32 ; 10] = [0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_10_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_10_10_10_10_10 , 0b00_11_11_11_11_11_11_11_11_11_10_10_10_10_10_10 , 0b00_10_11_11_11_11_11_11_11_10_10_10_10_10_11_10 , 0b00_10_10_11_11_11_11_11_10_10_10_10_10_11_11_10 , 0b00_10_10_10_11_11_11_10_10_10_10_10_11_11_11_10 , 0b00_10_10_10_10_11_10_10_10_10_10_11_11_11_11_10] ; let board0org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [coloridx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [coloridx] [1] [0]) ,] } ; let board1org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [emptyidx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [emptyidx] [1] [0]) ,] } ; let mut count_match : i32 = 0 ; for dir in 0 .. 2 { let mut board0 = board0org [dir] ; let mut board1 = board1org [dir] ; let boardf1 = _mm512_and_si512 (answer_color , board0) ; let boardf2 = _mm512_and_si512 (answer_empty , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [0] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; for i in 1 .. 10 { board0 = _mm512_slli_epi32 (board0 , 1) ; board1 = _mm512_slli_epi32 (board1 , 1) ; let boardf1 = _mm512_and_si512 (answer_color , board0) ; let boardf2 = _mm512_and_si512 (answer_empty , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [i] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; } } count_match > 0 }
}

macro_rules! check_patterndead4_avx512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patterndead4_avx512 in module {}", module_path!());
    };
}

mkfn!{
    check_patterndead4_avx512_introspect!();
    # [target_feature (enable = "avx512f,avx512bw,popcnt")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn check_patterndead4_avx512 (pos : & Pos , sd : Side) -> i32 { let coloridx = sd as usize ; let emptyidx = Color :: Empty as usize ; # [rustfmt :: skip] let answer_color : [__m512i ; 5] = [_mm512_set1_epi16 ((1 << 14) | (1 << 13) | (1 << 12) | (1 << 11)) , _mm512_set1_epi16 ((1 << 15) | (1 << 13) | (1 << 12) | (1 << 11)) , _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 12) | (1 << 11)) , _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 11)) , _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12))] ; # [rustfmt :: skip] let answer_empty : [__m512i ; 5] = [_mm512_set1_epi16 (1 << 15) , _mm512_set1_epi16 (1 << 14) , _mm512_set1_epi16 (1 << 13) , _mm512_set1_epi16 (1 << 12) , _mm512_set1_epi16 (1 << 11)] ; # [rustfmt :: skip] let answer = _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11)) ; # [rustfmt :: skip] let answer_mask : [__mmask32 ; 11] = [0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_10_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_10_10_10_10_10 , 0b00_11_11_11_11_11_11_11_11_11_10_10_10_10_11_10 , 0b00_10_11_11_11_11_11_11_11_10_10_10_10_11_11_10 , 0b00_10_10_11_11_11_11_11_10_10_10_10_11_11_11_10 , 0b00_10_10_10_11_11_11_10_10_10_10_11_11_11_11_10 , 0b00_10_10_10_10_11_10_10_10_10_11_11_11_11_11_10] ; let board0org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [coloridx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [coloridx] [1] [0]) ,] } ; let board1org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [emptyidx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [emptyidx] [1] [0]) ,] } ; let mut count_match : i32 = 0 ; for pattern in 0 .. 5 { for dir in 0 .. 2 { let mut board0 = board0org [dir] ; let mut board1 = board1org [dir] ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [0] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; for i in 1 .. 11 { board0 = _mm512_slli_epi32 (board0 , 1) ; board1 = _mm512_slli_epi32 (board1 , 1) ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [i] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; } } } count_match }
}

macro_rules! check_patternlive3_avx512_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_patternlive3_avx512 in module {}", module_path!());
    };
}

mkfn!{
    check_patternlive3_avx512_introspect!();
    # [target_feature (enable = "avx512f,avx512bw,popcnt")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn check_patternlive3_avx512 (pos : & Pos , sd : Side) -> i32 { let coloridx = sd as usize ; let emptyidx = Color :: Empty as usize ; # [rustfmt :: skip] let board0org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [coloridx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [coloridx] [1] [0])] } ; # [rustfmt :: skip] let board1org : [__m512i ; 2] = unsafe { [_mm512_loadu_epi32 (& pos . bitboard [emptyidx] [0] [0]) , _mm512_loadu_epi32 (& pos . bitboard [emptyidx] [1] [0])] } ; # [rustfmt :: skip] let answer_color : [__m512i ; 1] = [_mm512_set1_epi16 ((1 << 14) | (1 << 13) | (1 << 12))] ; # [rustfmt :: skip] let answer_empty : [__m512i ; 1] = [_mm512_set1_epi16 ((1 << 15) | (1 << 11))] ; # [rustfmt :: skip] let answer : __m512i = _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11)) ; let mut count_match : i32 = 0 ; # [rustfmt :: skip] let answer_mask : [__mmask32 ; 11] = [0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_11 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_10_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_10_10_10_10_10 , 0b00_11_11_11_11_11_11_11_11_11_10_10_10_10_11_10 , 0b00_10_11_11_11_11_11_11_11_10_10_10_10_11_11_10 , 0b00_10_10_11_11_11_11_11_10_10_10_10_11_11_11_10 , 0b00_10_10_10_11_11_11_10_10_10_10_11_11_11_11_10 , 0b00_10_10_10_10_11_10_10_10_10_11_11_11_11_11_10] ; for pattern in 0 .. 1 { for dir in 0 .. 2 { let mut board0 = board0org [dir] ; let mut board1 = board1org [dir] ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [0] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; for i in 1 .. 11 { board0 = _mm512_slli_epi32 (board0 , 1) ; board1 = _mm512_slli_epi32 (board1 , 1) ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [i] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; } } } # [rustfmt :: skip] let answer_color : [__m512i ; 2] = [_mm512_set1_epi16 ((1 << 14) | (1 << 12) | (1 << 11)) , _mm512_set1_epi16 ((1 << 14) | (1 << 13) | (1 << 11))] ; # [rustfmt :: skip] let answer_empty : [__m512i ; 2] = [_mm512_set1_epi16 ((1 << 15) | (1 << 13) | (1 << 10)) , _mm512_set1_epi16 ((1 << 15) | (1 << 12) | (1 << 10))] ; # [rustfmt :: skip] let answer : __m512i = _mm512_set1_epi16 ((1 << 15) | (1 << 14) | (1 << 13) | (1 << 12) | (1 << 11) | (1 << 10)) ; # [rustfmt :: skip] let answer_mask : [__mmask32 ; 10] = [0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_11_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_11_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_11_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_11_10_10_10_10 , 0b01_11_11_11_11_11_11_11_11_11_11_10_10_10_10_10 , 0b00_11_11_11_11_11_11_11_11_11_10_10_10_10_10_10 , 0b00_10_11_11_11_11_11_11_11_10_10_10_10_10_11_10 , 0b00_10_10_11_11_11_11_11_10_10_10_10_10_11_11_10 , 0b00_10_10_10_11_11_11_10_10_10_10_10_11_11_11_10 , 0b00_10_10_10_10_11_10_10_10_10_10_11_11_11_11_10] ; for pattern in 0 .. 2 { for dir in 0 .. 2 { let mut board0 = board0org [dir] ; let mut board1 = board1org [dir] ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [0] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; for i in 1 .. 10 { board0 = _mm512_slli_epi32 (board0 , 1) ; board1 = _mm512_slli_epi32 (board1 , 1) ; let boardf1 = _mm512_and_si512 (answer_color [pattern] , board0) ; let boardf2 = _mm512_and_si512 (answer_empty [pattern] , board1) ; let boardf = _mm512_or_si512 (boardf1 , boardf2) ; let temp_mask = _mm512_mask_cmpeq_epi16_mask (answer_mask [i] , answer , boardf) ; count_match += _popcnt32 (temp_mask as i32) ; } } } count_match }
}

macro_rules! check_x86_avx512_features_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_x86_avx512_features in module {}", module_path!());
    };
}

mkfn!{
    check_x86_avx512_features_introspect!();
    # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] fn check_x86_avx512_features () -> bool { is_x86_feature_detected ! ("avx512bw") && is_x86_feature_detected ! ("popcnt") }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { if check_x86_avx512_features () { println ! ("\n\nThe program is running with avx512f and avx512bw intrinsics\n\n") ; } else { println ! ("\n\nThe program is running with NO intrinsics.\n\n") ; } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] { println ! ("\n\nThe program is running with NO intrinsics.\n\n") ; } loop { let start = Instant :: now () ; println ! ("Hello, this is Connect5 (Outer-Open Gomoku)!") ; println ! ("Self-playing with search depth = 4") ; let test_state : [Color ; SQUARE_SIZE as usize] = [Color :: Empty ; SQUARE_SIZE as usize] ; let test_bitboard : [[[i32 ; 16] ; 2] ; 3] = [[[0 ; 16] ; 2] ; 3] ; let mut test1 = Pos { state : test_state , p_turn : Color :: Black , bitboard : test_bitboard , } ; test1 . init () ; let mut count : i32 = 0 ; for i in 0 .. (FILE_SIZE * RANK_SIZE) { let mut next_move : Move = square_make (1 , 7) ; if i > 0 { next_move = search (& test1 , - EVAL_INF , EVAL_INF , 4 , 0) ; } test1 . do_move (next_move) ; pos_disp (& test1) ; if pos_is_end (& test1) { println ! ("Game over!!!!!! at Move {i}") ; count = i + 1 ; break ; } } let duration = start . elapsed () ; println ! ("Average time for each move is: {:?}" , duration / count as u32) ; } }
}