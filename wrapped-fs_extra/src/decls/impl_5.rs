macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Error { # [doc = " Create a new fs_extra error from a kind of error error as well as an arbitrary error payload."] # [doc = ""] # [doc = "#Examples"] # [doc = " ```rust,ignore"] # [doc = ""] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::error::{Error, ErrorKind};"] # [doc = ""] # [doc = " errors can be created from strings"] # [doc = " let custom_error = Error::new(ErrorKind::Other, \"Other Error!\");"] # [doc = " // errors can also be created from other errors"] # [doc = " let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);"] # [doc = ""] # [doc = " ```"] pub fn new (kind : ErrorKind , message : & str) -> Error { Error { kind , message : message . to_string () , } } }
    };
}

impl_5!()