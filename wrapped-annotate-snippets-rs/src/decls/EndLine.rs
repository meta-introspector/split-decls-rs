macro_rules! EndLine {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq)] enum EndLine { Eof , Lf , Crlf , }
    };
}

EndLine!()