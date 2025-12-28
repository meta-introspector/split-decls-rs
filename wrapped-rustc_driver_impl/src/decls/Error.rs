macro_rules! Error {
    () => {
        # [derive (Debug)] enum Error { Utf8Error (String) , IOError (String , io :: Error) , ShellParseError (String) , }
    };
}

Error!()