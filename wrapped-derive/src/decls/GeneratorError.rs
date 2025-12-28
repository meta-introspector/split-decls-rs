macro_rules! GeneratorError {
    () => {
        # [derive (Error , Debug)] pub enum GeneratorError { # [error ("{0}")] Syn (# [from] syn :: Error) , # [error ("{0}")] Darling (# [from] darling :: Error) , }
    };
}

GeneratorError!()