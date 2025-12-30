// Generated macro for impl_963 (impl)
macro_rules! Depcrate_strategy_shuffleimpl_963 {
() => {
// Module: crate::strategy::shuffle
// Provides: {"impl_963"}
// Dependencies: {}
impl < V : ValueTree > ValueTree for ShuffleValueTree < V > where V :: Value : Shuffleable , { type Value = V :: Value ; fn current (& self) -> V :: Value { let mut value = self . inner . current () ; let len = value . shuffle_len () ; let max_swap = self . init_dist (len) ; if 0 == len || 0 == max_swap { return value ; } let mut rng = self . rng . clone () ; for start_index in 0 .. len - 1 { let end_index = rng . random_range (start_index .. len) ; if end_index - start_index <= max_swap { value . shuffle_swap (start_index , end_index) ; } } value } fn simplify (& mut self) -> bool { if self . simplifying_inner { self . inner . simplify () } else { self . force_init_dist () ; if self . dist . get_mut () . as_mut () . unwrap () . simplify () { true } else { self . simplifying_inner = true ; self . inner . simplify () } } } fn complicate (& mut self) -> bool { if self . simplifying_inner { self . inner . complicate () } else { self . force_init_dist () ; self . dist . get_mut () . as_mut () . unwrap () . complicate () } } }
};
}
