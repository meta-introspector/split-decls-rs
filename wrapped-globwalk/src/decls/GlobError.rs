macro_rules! GlobError {
    () => {
        # [doc = " Error from parsing globs."] # [derive (Debug)] pub struct GlobError (ignore :: Error) ;
    };
}

GlobError!()