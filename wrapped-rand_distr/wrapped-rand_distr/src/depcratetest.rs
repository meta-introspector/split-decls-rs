// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [macro_use] mod test { # [doc = " Construct a deterministic RNG with the given seed"] pub fn rng (seed : u64) -> impl rand :: RngCore { const INC : u64 = 11634580027462260723 ; rand_pcg :: Pcg32 :: new (seed , INC) } # [doc = " Assert that two numbers are almost equal to each other."] # [doc = ""] # [doc = " On panic, this macro will print the values of the expressions with their"] # [doc = " debug representations."] macro_rules ! assert_almost_eq { ($ a : expr , $ b : expr , $ prec : expr) => { let diff = ($ a - $ b) . abs () ; assert ! (diff <= $ prec , "assertion failed: `abs(left - right) = {:.1e} < {:e}`, \
                    (left: `{}`, right: `{}`)" , diff , $ prec , $ a , $ b) ; } ; } }
};
}
