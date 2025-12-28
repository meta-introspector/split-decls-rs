macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! context {
    () => {
        deps!();
        # [doc = " Create a new error from an input position, a static string and an existing error."] # [doc = " This is used mainly in the [context] combinator, to add user friendly information"] # [doc = " to errors when backtracking through a parse tree"] pub fn context < F > (context : & 'static str , parser : F) -> Context < F > { Context { context , parser } }
    };
}

context!()