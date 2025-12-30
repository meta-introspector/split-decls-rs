// Generated macro for pearson_correlation (function)
macro_rules! Depcrate_scoringpearson_correlation {
() => {
// Module: crate::scoring
// Provides: {"pearson_correlation"}
// Dependencies: {}
# [doc = " Calculates the pearson_correlation of the passed `(len, time)`-array."] # [allow (dead_code)] pub fn pearson_correlation (time_samples : & [(f64 , f64)]) -> (f64 , bool) { let mut vec = Vec :: with_capacity (time_samples . len ()) ; vec . extend (time_samples . iter () . cloned () . map (| (x , y) | [x , y])) ; let time_samples = Array2 :: from (vec) ; let time_samples = time_samples . t () ; let corr = time_samples . pearson_correlation () . expect ("no time samples given") [[1 , 0]] ; (corr , corr < super :: ACCEPTANCE_CORRELATION) }
};
}
