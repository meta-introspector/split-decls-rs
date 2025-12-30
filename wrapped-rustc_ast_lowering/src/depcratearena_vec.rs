// Generated macro for arena_vec (macro)
macro_rules! Depcratearena_vec {
() => {
// Module: crate
// Provides: {"arena_vec"}
// Dependencies: {}
macro_rules ! arena_vec { ($ this : expr ; $ ($ x : expr) ,*) => ($ this . arena . alloc_from_iter ([$ ($ x) ,*])) ; }
};
}
