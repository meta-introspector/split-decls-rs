macro_rules! DisplaySourceCodeError {
    () => {
        # [derive (Debug)] pub enum DisplaySourceCodeError { PathNotFound , Coroutine , OpaqueType , }
    };
}

DisplaySourceCodeError!();