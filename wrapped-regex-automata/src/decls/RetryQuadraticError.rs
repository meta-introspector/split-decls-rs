macro_rules! RetryQuadraticError {
    () => {
        # [doc = " An error that occurs when potential quadratic behavior has been detected"] # [doc = " when applying either the \"reverse suffix\" or \"reverse inner\" optimizations."] # [doc = ""] # [doc = " When this error occurs, callers should abandon the \"reverse\" optimization"] # [doc = " and use a normal forward search."] # [derive (Debug)] pub (crate) struct RetryQuadraticError (()) ;
    };
}

RetryQuadraticError!()