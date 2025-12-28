macro_rules! Error {
    () => {
        # [doc = " Command Line Argument Parser Error"] # [doc = ""] # [doc = " See [`Command::error`] to create an error."] # [doc = ""] # [doc = " [`Command::error`]: crate::Command::error"] pub type Error = error :: Error < error :: DefaultFormatter > ;
    };
}

Error!()