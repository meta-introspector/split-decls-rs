// Generated macro for round32 (function)
macro_rules! Depcrateround32 {
() => {
// Module: crate
// Provides: {"round32"}
// Dependencies: {}
# [inline (always)] fn round32 < M : Machine > ((mut a , mut b , mut c , mut d) : (M :: u32x4 , M :: u32x4 , M :: u32x4 , M :: u32x4) , m0 : M :: u32x4 , m1 : M :: u32x4 ,) -> (M :: u32x4 , M :: u32x4 , M :: u32x4 , M :: u32x4) { a += m0 ; a += b ; d ^= a ; d = d . rotate_each_word_right16 () ; c += d ; b ^= c ; b = b . rotate_each_word_right12 () ; a += m1 ; a += b ; d ^= a ; d = d . rotate_each_word_right8 () ; c += d ; b ^= c ; b = b . rotate_each_word_right7 () ; (a , b , c , d) }
};
}
