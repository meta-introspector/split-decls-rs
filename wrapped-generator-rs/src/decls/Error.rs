macro_rules! deps {
    () => {
        Context!();
        Stack!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " yield panic error types"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Error { # [doc = " Done panic"] Done , # [doc = " Cancel panic"] Cancel , # [doc = " Type mismatch panic"] TypeErr , # [doc = " Stack overflow panic"] StackErr , # [doc = " Wrong Context panic"] ContextErr , }
    };
}

Error!();