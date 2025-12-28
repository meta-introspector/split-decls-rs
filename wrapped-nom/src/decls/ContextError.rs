macro_rules! ContextError {
    () => {
        # [doc = " This trait is required by the `context` combinator to add a static string"] # [doc = " to an existing error"] pub trait ContextError < I > : Sized { # [doc = " Creates a new error from an input position, a static string and an existing error."] # [doc = " This is used mainly in the [context] combinator, to add user friendly information"] # [doc = " to errors when backtracking through a parse tree"] fn add_context (_input : I , _ctx : & 'static str , other : Self) -> Self { other } }
    };
}

ContextError!()