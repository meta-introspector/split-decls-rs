// Generated macro for write_n (function)
macro_rules! Depcrate_nestedwrite_n {
() => {
// Module: crate::nested
// Provides: {"write_n"}
// Dependencies: {}
# [doc = " Write `n` times `c` to `out`."] fn write_n < W : std :: io :: Write > (out : & mut W , c : char , n : usize) -> std :: io :: Result < () > { for _ in 0 .. n { write ! (out , "{c}") ? ; } Ok (()) }
};
}
