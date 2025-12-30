// Generated macro for process_results (function)
macro_rules! Depcrate_process_results_implprocess_results {
() => {
// Module: crate::process_results_impl
// Provides: {"process_results"}
// Dependencies: {}
# [doc = " “Lift” a function of the values of an iterator so that it can process"] # [doc = " an iterator of `Result` values instead."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::process_results`]."] pub fn process_results < I , F , T , E , R > (iterable : I , processor : F) -> Result < R , E > where I : IntoIterator < Item = Result < T , E > > , F : FnOnce (ProcessResults < I :: IntoIter , E >) -> R , { let iter = iterable . into_iter () ; let mut error = Ok (()) ; let result = processor (ProcessResults { error : & mut error , iter , }) ; error . map (| _ | result) }
};
}
