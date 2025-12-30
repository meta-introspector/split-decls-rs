// Generated macro for force_eval (macro)
macro_rules! Depcrate_mathforce_eval {
() => {
// Module: crate::math
// Provides: {"force_eval"}
// Dependencies: {}
macro_rules ! force_eval { ($ e : expr) => { unsafe { :: core :: ptr :: read_volatile (&$ e) } } ; }
};
}
