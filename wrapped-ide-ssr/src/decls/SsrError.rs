macro_rules! SsrError {
    () => {
        # [derive (Debug , PartialEq)] pub struct SsrError (pub (crate) String) ;
    };
}

SsrError!();