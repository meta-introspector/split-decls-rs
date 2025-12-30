// Generated macro for saturating_is_num (function)
macro_rules! Depcratesaturating_is_num {
() => {
// Module: crate
// Provides: {"saturating_is_num"}
// Dependencies: {}
# [test] # [cfg (has_num_saturating)] fn saturating_is_num () { fn require_num < T : Num > (_ : & T) { } require_num (& core :: num :: Saturating (42_u32)) ; require_num (& core :: num :: Saturating (- 42)) ; }
};
}
