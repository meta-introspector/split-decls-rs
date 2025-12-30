// Generated macro for impl_127 (impl)
macro_rules! Depcrate_betaimpl_127 {
() => {
// Module: crate::beta
// Provides: {"impl_127"}
// Dependencies: {}
impl < F > Beta < F > where F : Float , Open01 : Distribution < F > , { # [doc = " Construct an object representing the `Beta(alpha, beta)`"] # [doc = " distribution."] pub fn new (alpha : F , beta : F) -> Result < Beta < F > , Error > { if ! (alpha > F :: zero ()) { return Err (Error :: AlphaTooSmall) ; } if ! (beta > F :: zero ()) { return Err (Error :: BetaTooSmall) ; } let (a0 , b0) = (alpha , beta) ; let (a , b , switched_params) = if a0 < b0 { (a0 , b0 , false) } else { (b0 , a0 , true) } ; if a > F :: one () { let alpha = a + b ; let two = F :: from (2.) . unwrap () ; let beta_numer = alpha - two ; let beta_denom = two * a * b - alpha ; let beta = (beta_numer / beta_denom) . sqrt () ; let gamma = a + F :: one () / beta ; Ok (Beta { a , b , switched_params , algorithm : BetaAlgorithm :: BB (BB { alpha , beta , gamma }) , }) } else { let (a , b , switched_params) = (b , a , ! switched_params) ; let alpha = a + b ; let beta = F :: one () / b ; let delta = F :: one () + a - b ; let kappa1 = delta * (F :: from (1. / 18. / 4.) . unwrap () + F :: from (3. / 18. / 4.) . unwrap () * b) / (a * beta - F :: from (14. / 18.) . unwrap ()) ; let kappa2 = F :: from (0.25) . unwrap () + (F :: from (0.5) . unwrap () + F :: from (0.25) . unwrap () / delta) * b ; Ok (Beta { a , b , switched_params , algorithm : BetaAlgorithm :: BC (BC { alpha , beta , kappa1 , kappa2 , }) , }) } } }
};
}
