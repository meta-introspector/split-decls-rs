macro_rules! deps {
    () => {
        InferCtxt!();
    };
}

macro_rules! FromSolverError {
    () => {
        deps!();
        pub trait FromSolverError < 'tcx , E > : Debug + 'tcx { fn from_solver_error (infcx : & InferCtxt < 'tcx > , error : E) -> Self ; }
    };
}

FromSolverError!();