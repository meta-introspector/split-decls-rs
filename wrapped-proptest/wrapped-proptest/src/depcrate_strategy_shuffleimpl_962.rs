// Generated macro for impl_962 (impl)
macro_rules! Depcrate_strategy_shuffleimpl_962 {
() => {
// Module: crate::strategy::shuffle
// Provides: {"impl_962"}
// Dependencies: {}
impl < V : ValueTree > ShuffleValueTree < V > where V :: Value : Shuffleable , { fn init_dist (& self , dflt : usize) -> usize { if self . dist . get () . is_none () { self . dist . set (Some (num :: usize :: BinarySearch :: new (dflt))) ; } self . dist . get () . unwrap () . current () } fn force_init_dist (& self) { if self . dist . get () . is_none () { self . init_dist (self . current () . shuffle_len ()) ; } } }
};
}
