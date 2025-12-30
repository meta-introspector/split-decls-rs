// Generated macro for round64 (function)
macro_rules! Depcrateround64 {
() => {
// Module: crate
// Provides: {"round64"}
// Dependencies: {}
# [inline (always)] fn round64 < M : Machine > ((mut a , mut b , mut c , mut d) : (M :: u64x4 , M :: u64x4 , M :: u64x4 , M :: u64x4) , m0 : M :: u64x4 , m1 : M :: u64x4 ,) -> (M :: u64x4 , M :: u64x4 , M :: u64x4 , M :: u64x4) { a += m0 ; a += b ; d ^= a ; d = d . rotate_each_word_right32 () ; c += d ; b ^= c ; b = b . rotate_each_word_right25 () ; a += m1 ; a += b ; d ^= a ; d = d . rotate_each_word_right16 () ; c += d ; b ^= c ; b = b . rotate_each_word_right11 () ; (a , b , c , d) }
};
}
