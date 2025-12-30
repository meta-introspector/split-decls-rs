// Generated macro for Resamples (struct)
macro_rules! Depcrate_stats_bivariate_resamplesResamples {
() => {
// Module: crate::stats::bivariate::resamples
// Provides: {"Resamples"}
// Dependencies: {}
pub struct Resamples < 'a , X , Y > where X : 'a + Float , Y : 'a + Float , { rng : Rng , data : (& 'a [X] , & 'a [Y]) , stage : Option < (Vec < X > , Vec < Y >) > , }
};
}
