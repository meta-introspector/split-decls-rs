macro_rules! HigherRankedErrorCause {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum HigherRankedErrorCause { # [note (borrowck_could_not_prove)] CouldNotProve { predicate : String } , # [note (borrowck_could_not_normalize)] CouldNotNormalize { value : String } , }
    };
}

HigherRankedErrorCause!()