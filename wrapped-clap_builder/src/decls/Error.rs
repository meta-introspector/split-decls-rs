macro_rules! deps {
    () => {
        Parser!();
        ErrorInner!();
        Command!();
        ErrorFormatter!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Command Line Argument Parser Error"] # [doc = ""] # [doc = " See [`Command::error`] to create an error."] # [doc = ""] # [doc = " [`Command::error`]: crate::Command::error"] pub struct Error < F : ErrorFormatter = DefaultFormatter > { inner : Box < ErrorInner > , phantom : std :: marker :: PhantomData < F > , }
    };
}

Error!()