macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Chain {
    () => {
        deps!();
        # [doc = " Iterator of a chain of source errors."] # [doc = ""] # [doc = " This type is the iterator returned by [`Error::chain`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use anyhow::Error;"] # [doc = " use std::io;"] # [doc = ""] # [doc = " pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {"] # [doc = "     for cause in error.chain() {"] # [doc = "         if let Some(io_error) = cause.downcast_ref::<io::Error>() {"] # [doc = "             return Some(io_error.kind());"] # [doc = "         }"] # [doc = "     }"] # [doc = "     None"] # [doc = " }"] # [doc = " ```"] # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] # [derive (Clone)] pub struct Chain < 'a > { state : crate :: chain :: ChainState < 'a > , }
    };
}

Chain!()