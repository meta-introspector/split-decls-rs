// Generated macro for do_naive_diff (function)
macro_rules! Depcratedo_naive_diff {
() => {
// Module: crate
// Provides: {"do_naive_diff"}
// Dependencies: {}
fn do_naive_diff < 'a , T , F , U > (left : & 'a [T] , right : & 'a [T] , mapper : F , diff : & mut Vec < Result < U > >) where T : PartialEq , F : Fn (& 'a T) -> U , { let mut table = Vec2 :: new (0u32 , [left . len () + 1 , right . len () + 1]) ; for (i , l) in left . iter () . enumerate () { for (j , r) in right . iter () . enumerate () { table . set ([i + 1 , j + 1] , if l == r { table . get ([i , j]) + 1 } else { * table . get ([i , j + 1]) . max (table . get ([i + 1 , j])) } ,) ; } } let start = diff . len () ; let mut i = table . len [0] - 1 ; let mut j = table . len [1] - 1 ; loop { if j > 0 && (i == 0 || table . get ([i , j]) == table . get ([i , j - 1])) { j -= 1 ; diff . push (Result :: Right (mapper (& right [j]))) ; } else if i > 0 && (j == 0 || table . get ([i , j]) == table . get ([i - 1 , j])) { i -= 1 ; diff . push (Result :: Left (mapper (& left [i]))) ; } else if i > 0 && j > 0 { i -= 1 ; j -= 1 ; diff . push (Result :: Both (mapper (& left [i]) , mapper (& right [j]))) ; } else { break ; } } diff [start ..] . reverse () ; }
};
}
