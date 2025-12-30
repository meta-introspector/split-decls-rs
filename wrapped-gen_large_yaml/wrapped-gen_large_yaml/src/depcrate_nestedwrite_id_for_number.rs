// Generated macro for write_id_for_number (function)
macro_rules! Depcrate_nestedwrite_id_for_number {
() => {
// Module: crate::nested
// Provides: {"write_id_for_number"}
// Dependencies: {}
# [doc = " Create a valid identifier for the given number."] fn write_id_for_number < W : std :: io :: Write > (out : & mut W , mut n : usize) -> std :: io :: Result < () > { const DIGITS : & [u8] = b"_abcdefghijklmnopqrstuvwxyz" ; n += 1 ; while n > 0 { write ! (out , "{}" , DIGITS [n % DIGITS . len ()] as char) ? ; n /= DIGITS . len () ; } Ok (()) }
};
}
