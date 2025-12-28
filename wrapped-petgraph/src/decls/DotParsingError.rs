macro_rules! DotParsingError {
    () => {
        # [derive (Debug)] pub struct DotParsingError { error : Box < ParsingError > , }
    };
}

DotParsingError!();