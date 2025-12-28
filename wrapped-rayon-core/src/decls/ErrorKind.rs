macro_rules! ErrorKind {
    () => {
        # [derive (Debug)] enum ErrorKind { GlobalPoolAlreadyInitialized , CurrentThreadAlreadyInPool , IOError (io :: Error) , }
    };
}

ErrorKind!();