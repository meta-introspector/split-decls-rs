// Generated macro for num_threads (function)
macro_rules! Depcrate_parallelnum_threads {
() => {
// Module: crate::parallel
// Provides: {"num_threads"}
// Dependencies: {}
# [doc = " Returns the amount of threads the system can effectively use as the amount of its logical cores."] # [doc = ""] # [doc = " Only available with the `parallel` feature toggle set."] # [cfg (feature = "parallel")] pub fn num_threads (thread_limit : Option < usize >) -> usize { let logical_cores = std :: thread :: available_parallelism () . map_or (1 , Into :: into) ; thread_limit . map_or (logical_cores , | l | if l == 0 { logical_cores } else { l }) }
};
}
