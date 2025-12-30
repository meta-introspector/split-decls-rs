// Generated macro for random_iter (function)
macro_rules! Depcraterandom_iter {
() => {
// Module: crate
// Provides: {"random_iter"}
// Dependencies: {}
# [doc = " Return an iterator over [`random()`] variates"] # [doc = ""] # [doc = " This function is shorthand for"] # [doc = " <code>[rng()].[random_iter](Rng::random_iter)()</code>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let v: Vec<i32> = rand::random_iter().take(5).collect();"] # [doc = " println!(\"{v:?}\");"] # [doc = " ```"] # [cfg (feature = "thread_rng")] # [inline] pub fn random_iter < T > () -> distr :: Iter < StandardUniform , rngs :: ThreadRng , T > where StandardUniform : Distribution < T > , { rng () . random_iter () }
};
}
