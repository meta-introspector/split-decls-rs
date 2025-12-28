macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let msg = match self { Self :: Parse (msg) => msg . clone () , Self :: MustBeStringLiteral => "Format string must be a string literal" . to_owned () , Self :: UnableToParseTag (tag) => format ! ("Unable to parse the tag {}" , tag) , Self :: ParseTag (detail) => detail . clone () , Self :: UnclosedPlaceholder => "Unclosed placeholder" . to_owned () , Self :: UnclosedTag => "Unclosed color tag" . to_owned () , Self :: NoTagToClose => "No color tag to close" . to_owned () , Self :: MismatchCloseTag (tag1 , tag2) => { format ! ("Mismatch close tag between {} and {}" , tag1 , tag2) } Self :: TooManyArgs => "Too many arguments" . to_owned () , } ; write ! (f , "{}" , msg) } }
    };
}

impl_75!()