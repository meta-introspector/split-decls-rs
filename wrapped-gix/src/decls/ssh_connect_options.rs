macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ssh_connect_options {
    () => {
        deps!();
        # [doc = ""] pub mod ssh_connect_options { # [doc = " The error produced when obtaining ssh connection configuration."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error (transparent)] pub struct Error (# [from] super :: key :: GenericErrorWithValue) ; }
    };
}

ssh_connect_options!();