macro_rules! deps {
    () => {
        ProcessResults!();
    };
}

macro_rules! process_results {
    () => {
        deps!();
        # [doc = " “Lift” a function of the values of an iterator so that it can process"] # [doc = " an iterator of `Result` values instead."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::process_results`]."] pub fn process_results < I , F , T , E , R > (iterable : I , processor : F) -> Result < R , E > where I : IntoIterator < Item = Result < T , E > > , F : FnOnce (ProcessResults < I :: IntoIter , E >) -> R , { let iter = iterable . into_iter () ; let mut error = Ok (()) ; let result = processor (ProcessResults { error : & mut error , iter , }) ; error . map (| _ | result) }
    };
}

process_results!();